const glob = require('glob');
const path = require('path');
const { execSync } = require('child_process');

const args = process.argv.slice(2);
const target = args.shift() || '.';

glob.glob(target, (_, files) => {
  const logs = files
    .map(f => execSync(
      `git --no-pager log --reverse --pretty=format:'{"hash": "%h", "message": "%s", "date": "%ad", "file": "${f}"}' --date=format:'%Y-%m-%d' "${f}"`
      ).toString().split('\n')[0])
    .filter(log=>log)
    .map(log => JSON.parse(log))
    .reduce((acc, log) => {
      if (!acc[log.date]) {
        acc[log.date] = [];
      }
      acc[log.date].push(log);
      return acc;
    }, {});
  console.log(JSON.stringify(logs));
});
