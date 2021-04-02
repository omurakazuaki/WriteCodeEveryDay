const path = require('path');
const readStdin = require('../libs/readStdin');

const header = ['Sun','Mon','Tue','Wed','Thu','Fri','Sat'];

(async () => {
  const rawData = JSON.parse(await readStdin());

  const calendars = Object.keys(rawData).reduce((acc, dateAsStr) => {
    const [year, month, date] = dateAsStr.split('-');
    if (!acc[year]) {
      acc[year] = [];
    }
    const lastDay = new Date(year, month, 0);
    const blankTail = 6 - lastDay.getDay();
    const blankHead = 7 - (lastDay.getDate() + blankTail) % 7;
    if (!acc[year][Number(month)]) {
      const calendar = new Array(lastDay.getDate()).fill(null).map((_, i) => ({date: i + 1, log: '-'}));
      calendar.unshift(...new Array(blankHead).fill(null));
      calendar.push(...new Array(blankTail).fill(null));
      acc[year][Number(month)] = calendar;
    }
    acc[year][Number(month)][blankHead + Number(date) - 1].log = rawData[dateAsStr].map(log=>`[${log.message}](${path.dirname(log.file)})`).join('<br>');
    return acc;
  }, {});

  const chunk = ([...array], size = 1) => {
    return array.reduce((acc, _, index) => index % size ? acc : [...acc, array.slice(index, index + size)], []);
  }

  const markDown = Object.keys(calendars).sort().reduce((md, year) => {
    md += '### ' + year + '\n\n';
    return calendars[year].reduce((mdYear, month, i) => {
      if (month) {
        mdYear += [
          `#### ${i}`,
          header.join('|'),
          header.map(_=>'-').join('|'),
          ...chunk(month.map(data => data ? `${data.date}${data.log ? ' 🍺' : ''}<br>${data.log}` : '-'), header.length)
          .map(week => [...week].join('|'))
        ].join('\n') + '\n';
      }
      return mdYear;
    }, md);
  }, '');
  console.log(markDown);
})();
