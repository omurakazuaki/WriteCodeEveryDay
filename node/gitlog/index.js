const glob = require('glob');
const Git = require('../libs/git');

const args = process.argv.slice(2);
const target = args.shift() || '.';
const git = new Git();

glob.glob(target, (_, files) => {
  Promise.all(
    files.map(f => git.log(f))
  ).then(result => console.log(JSON.stringify(
    result
      .filter(logs=>logs)
      .map(logs=>logs.pop())
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
