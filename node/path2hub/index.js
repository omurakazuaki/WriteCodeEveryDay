const fs = require('fs');
const path = require('path');
const Git = require('../libs/git');

const args = process.argv.slice(2);
const target = args.shift();
const start = args.shift();
const end = args.shift();
const protocol = 'https';

(async() => {
  const absoluteTarget = path.resolve(target);
  if (!fs.existsSync(absoluteTarget)) {
    process.exit(1);
  }
  const type = fs.statSync(absoluteTarget).isFile() ? 'blob' : 'tree';
  const git = new Git(absoluteTarget);
  const hash = git.hash();
  const config = git.config();
  const githubUrl = config.remote.origin.url;
  const githubWebUrl = githubUrl.replace(/^(ssh|https):\/\/(git@)?(.+)\.git$/, '$3');
  const topLevel = path.dirname(git.gitDir);
  const filePath = encodeURI(absoluteTarget.replace(topLevel, ''));
  const anchor = start ? `#L${start}` + (end ? `-#L${end}` : '') : ''
  console.log(`${protocol}://${path.join(githubWebUrl, type, hash, filePath)}${anchor}`);
  if (type === 'blob') {
    console.log(`${protocol}://${path.join(githubWebUrl, 'blame', hash, filePath)}`);
  }
  //console.log(`${protocol}://${path.join(githubWebUrl, 'commits', hash, filePath)}`);
})();

