const fs = require('fs');
const path = require('path');
const util = require('util');
const { exec } = require('child_process');
const zlib = require('zlib');

const execAsync = util.promisify(exec);
const inflateAsync = util.promisify(zlib.inflate);

module.exports = class Git {
  gitDir;
  packedRefs;
  constructor(baseDir=process.cwd()) {
    this.gitDir = Git.findGitDir(baseDir);
    this.packedRefs = Git.packedRefs(this.gitDir);
  }

  static findGitDir(targetDir) {
    const absoluteTarget = path.resolve(targetDir);
    const parentDir = path.dirname(absoluteTarget);
    if (targetDir == '.' || targetDir == '/') {
      return null;
    } else if (fs.statSync(absoluteTarget).isFile()) {
      return Git.findGitDir(parentDir);
    } else {
      const gitDir = fs.readdirSync(absoluteTarget)
        .map(f => path.join(absoluteTarget, f))
        .find(f => fs.statSync(f).isDirectory() && f.match(/\/.git$/));
      if (gitDir) {
        return gitDir;
      }
      return Git.findGitDir(parentDir);
    }
  }

  static packedRefs(gitDir) {
    return fs.readFileSync(path.join(gitDir, 'packed-refs')).toString().split('\n')
      .map(line => line.replace(/#.*$/, '').trim())
      .filter(line => line.length)
      .reduce((acc, line) => {
        const [ _, key, value ] = line.match(/([a-f0-9]{40}) (.*)/);
        acc[key] = value;
        return acc;
      }, {});
  }

  log(file, branch='') {
    return execAsync(
      `git --no-pager log --pretty=format:'{"hash": "%H", "message": "%s", "date": "%ad"}' --date=format:'%Y-%m-%d' ${branch} "${file}"`
    ).then(result => {
      const stdout = result.stdout.toString().trim();
      if (!stdout) {
        return undefined;
      }
      return stdout.split('\n').map(s=>{
        const o = JSON.parse(s)
        o.file = file;
        return o
      });
    });
  }

  head() {
    return fs.readFileSync(path.join(this.gitDir, 'HEAD'))
      .toString()
      .trim();
  }

  hash(ref=this.head()) {
    if (ref.startsWith('ref: ')) {
      const refFile = path.join(this.gitDir, ref.replace('ref: ', ''));
      if (!fs.existsSync(refFile)) {
        return undefined;
      }
      return this.hash(fs.readFileSync(refFile)
        .toString()
        .trim());
    } else {
      return ref;
    }
  }

  object(hash) {
    if (this.packedRefs[hash]) {
      // TODO: hack
      return { hash, type: 'packed' };
    }
    const compressed = fs.readFileSync(this.objectPath(hash));
    return inflateAsync(compressed)
      .then(buff => {
        // [type] [content size]\0[content]
        const headerEndIndex = buff.findIndex(n=>n===0);
        const headerByteArray = buff.slice(0, headerEndIndex);
        const contentByteArray = buff.slice(headerEndIndex + 1);
        const [ type, _ ] = headerByteArray.toString().split(' ');
        return {
          hash,
          type,
          content: type === 'commit' ? Git.parseCommitObject(contentByteArray)
                 : type === 'tree'   ? Git.parseTreeObject(contentByteArray)
                 : type === 'blob'   ? contentByteArray.toString()
                 : undefined
        };
      });
  }

  commitObject(commitHash=this.hash()) {
    return this.object(commitHash);
  }

  async commitHistory(latestCommitHash=this.hash()) {
    let commitHash = latestCommitHash;
    const history = [];
    while (true) {
      const commit = await this.commitObject(commitHash);
      if (!commit || !commit.content) {
        return history;
      }
      commitHash = commit.content.parent;
      history.push(commit);
    }
  }

  async findObject(filepath, commitHash=this.hash()) {
    const commit = await this.commitObject(commitHash);
    let obj = await this.object(commit.content.tree);
    const names = filepath.split('/');
    while (names.length) {
      const name = names.shift();
      if (obj.type !== 'tree') {
        throw new Error(`Illegal path: ${filepath}`);
      }
      const entry = obj.content.find(e=>e.name === name);
      if (!entry) {
        return undefined;
      }
      obj = await this.object(entry.hash);
    }
    return obj;
  }

  objectPath(hash) {
    return path.join(this.gitDir, 'objects', hash.substr(0,2), hash.substr(2));
  }

  static parseCommitObject(contentByteArray) {
    const lines = contentByteArray.toString();
    const content = {};
    const parseDetail = (str) => {
      const [ _, name, email, date ] = str.match(/(.+?) <(.+?)> (\d+) [\+\-]\d{4}/);
      return {name, email, date: new Date(Number(date) * 1000)};
    };
    content.tree = lines.match(/^tree (.+)\n/)[1];
    content.parent = lines.match(/\nparent (.+)\n/)[1];
    content.author = parseDetail(lines.match(/\nauthor (.+)\n/)[1]);
    content.committer = parseDetail(lines.match(/\ncommitter (.+)/)[1]);
    content.message = lines.match(/\n\n(.+)\n/)[1];
    return content;
  }

  static parseTreeObject(contentByteArray) {
    // [mode] [file/folder name]\0[SHA-1 of referencing blob or tree]
    const entries = [];
    for (let i = 0, used = 0; i < contentByteArray.length; i++) {
      const b = contentByteArray[i];
      if (b === 32) {
        entries.push({
          mode: contentByteArray.toString(undefined, used, i).padStart(6, '0')
        });
        used = i + 1;
      } else if (b === 0) {
        entries[entries.length-1].name = contentByteArray.toString(undefined, used, i);
        used = i + 1;
        i += 21;
        entries[entries.length-1].hash = contentByteArray.toString('hex', used, i)
        used = i;
      }
    }
    return entries;
  }

  config() {
    return fs.readFileSync(path.join(this.gitDir, 'config'))
      .toString()
      .trim()
      .split('\n')
      .map(l=>l.trim())
      .reduce((acc, l) => {
        const section = l.match(/^\[(\S*) ?"?(\S+?)?"?\]$/);
        const keyValue = l.match(/^(\S*)\s*=\s*"?(\S*?)"?$/);
        if (section) {
          acc.push({section: section[1]})
          if (section[2]) {
            acc[acc.length-1].subSection = section[2];
          }
        } else if (keyValue) {
          acc[acc.length-1][keyValue[1]] = keyValue[2];
        }
        return acc;
      }, [])
      .reduce((acc, obj) => {
        const keyValues = Object.keys(obj)
          .filter(k => !['section', 'subSection'].includes(k))
          .reduce((kv, k) => {
            kv[k] = obj[k];
            return kv;
          }, {});
        if (!acc[obj.section]) {
          acc[obj.section] = {};
        }
        if (obj.subSection) {
          acc[obj.section][obj.subSection] = keyValues;
        } else {
          acc[obj.section] = keyValues;
        }
        return acc;
      }, {});
  }

}
