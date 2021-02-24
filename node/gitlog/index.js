const glob = require('glob');
const { exec } = require('child_process');
const util = require('util');

const args = process.argv.slice(2);
const target = args.shift() || '.';

execAsync = util.promisify(exec);

glob.glob(target, (_, files) => {
  Promise.all(
    files.map(f => execAsync(
      `git --no-pager log --reverse --pretty=format:'{"hash": "%h", "message": "%s", "date": "%ad", "file": "${f}"}' --date=format:'%Y-%m-%d' "${f}"`
    ))
  ).then(result => console.log(JSON.stringify(
    result
      .map(r=>r.stdout.toString().split('\n')[0])
      .filter(log=>log)
      .map(log => JSON.parse(log))
      .reduce((acc, log) => {
        if (!acc[log.date]) {
          acc[log.date] = [];
        }
        acc[log.date].push(log);
        return acc;
      }, {})
    ))
  );
});
