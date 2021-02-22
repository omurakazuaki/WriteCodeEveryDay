const glob = require('glob');

const args = process.argv.slice(2);
const targetFilePattern = args.shift();

glob.glob(targetFilePattern, (_, files) => {
  const calendars = files.sort().map(f => {
    const [_, year, month] = f.match(/\/([0-9]{4})\/([0-9]{2})\//);
    const lastDay = new Date(year, month, 0);
    const data = require(f);
    const calender = new Array(lastDay.getDate()).fill(null).map((_, i) => {
      return {day: i + 1, title: data[i + 1] || '-'};
    });
    const blankTail = 6 - lastDay.getDay();
    const blankHead = 7 - (lastDay.getDate() + blankTail) % 7;
    calender.unshift(...new Array(blankHead).fill(null));
    calender.push(...new Array(blankTail).fill(null));

    return { year, month, calender }
  }).reduce((result, data) => {
    if (!result[data.year]) {
      result[data.year] = [];
    }
    result[data.year][Number(data.month)] = data.calender;
    return result;
  }, {});

  const chunk = ([...array], size = 1) => {
    return array.reduce((acc, _, index) => index % size ? acc : [...acc, array.slice(index, index + size)], []);
  }

  const markDown = Object.keys(calendars).sort().reduce((md, year) => {
    md += '### ' + year + '\n\n';
    return calendars[year].reduce((mdYear, month, i) => {
      if (month) {
        mdYear += '#### ' + i + '\n'
         + '|Sun|Mon|Tue|Wed|Thu|Fri|Sat|\n'
         + '|-|-|-|-|-|-|-|\n'
         + [...chunk(month.map(date => date ? date.day + (date.title != '-' ? ' 🍺' : '') + '<br>' + date.title : '-'), 7), '']
          .map(week => ['', ...week].join('|')).join('|\n') + '\n';
      }
      return mdYear;
    }, md);
  }, '');
  console.log(markDown);
});
