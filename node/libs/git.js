const fs = require('fs');
const path = require('path');
const util = require('util');
const { exec } = require('child_process');

const execAsync = util.promisify(exec);

module.exports = class Git {
  gitDir;
  constructor(baseDir=process.cwd()) {
    this.gitDir = Git.findGitDir(baseDir);
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

  hash(ref) {
    const refFile = path.join(this.gitDir, ref);
    if (!fs.existsSync(refFile)) {
      return undefined;
    }
    return fs.readFileSync(refFile)
      .toString()
      .trim();
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
      .reduce((acc, l) => {
        const section = l.section;
        const subSection = l.subSection;
        delete(l.section);
        delete(l.subSection);
        if (!acc[section]) {
          acc[section] = {};
        }
        if (subSection) {
          acc[section][subSection] = l;
        } else {
          acc[section] = l;
        }
        return acc;
      }, {})
  }

}
