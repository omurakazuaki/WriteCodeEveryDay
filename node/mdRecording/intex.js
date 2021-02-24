const config = require('./config');
const readStdin = require('../libs/readStdin');

(async () => {
  const rawData = JSON.parse(await readStdin());

  const records = Object.keys(rawData)
    .map(dateAsStr=>new Date(dateAsStr))
    .sort((a, b) => a - b)
    .reduce((acc, date) => {
      const current = acc[acc.length - 1];
      if (current.length == 0 || date - current[current.length - 1] <= 24*60*60*1000) {
        current.push(date);
      } else {
        acc.push([date]);
      }
      return acc;
    }, [[]]);
  const highestRecord = records.slice().sort((a, b) => b.length - a.length)[0].length;
  const latestRecord = records[records.length - 1].length;
  const nextAnniversary = config.anniversaries.find(a=> a > latestRecord);
  console.log(`- 🥇 Highest record: **${highestRecord}** days
- 😊 Latest record: **${latestRecord}** days ( **${nextAnniversary - latestRecord}** days to reach 🎉**${nextAnniversary}** days anniversary )`);

})();
