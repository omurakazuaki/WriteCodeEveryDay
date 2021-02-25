const fs = require('fs');
const path = require('path');
const Git = require('../libs/git');

const args = process.argv.slice(2);
const target = args.shift();
const protocol = 'https';

(async() => {
  const absoluteTarget = path.resolve(target);
  if (!fs.existsSync(absoluteTarget)) {
    process.exit(1);
  }
  const git = new Git(absoluteTarget);
  const hash = git.hash();
  const config = git.config();
  const githubUrl = config.remote.origin.url;
  const githubWebUrl = githubUrl.replace(/^(ssh|https):\/\/(git@)?(.+)\.git$/, '$3');
  const topLevel = path.dirname(git.gitDir);
  const filePath = encodeURI(absoluteTarget.replace(topLevel, ''));
  console.log(`${protocol}://${path.join(githubWebUrl, 'blob', hash, filePath)}`);
  console.log(`${protocol}://${path.join(githubWebUrl, 'blame', hash, filePath)}`);
  //console.log(`${protocol}://${path.join(githubWebUrl, 'commits', hash, filePath)}`);
})();

