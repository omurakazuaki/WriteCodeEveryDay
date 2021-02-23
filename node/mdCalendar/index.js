const path = require('path');

const readStdIn = () => {
  return new Promise((resolve, _) => {
    let buffer = '';
    process.stdin
      .resume()
      .setEncoding('utf8')
      .on('readable', () => {
        while ((chunk = process.stdin.read()) !== null) {
          buffer += chunk;
        }
      })
      .on('end', () => resolve(buffer));
  });
};

(async () => {
  const rawData = JSON.parse(await readStdIn());

  const calendars = Object.keys(rawData).reduce((acc, dateAsStr) => {
    const [year, month, date] = dateAsStr.split('-');
    if (!acc[year]) {
      acc[year] = [];
    }
    if (!acc[year][Number(month)]) {
      const lastDay = new Date(year, month, 0);
      const calendar = new Array(lastDay.getDate()).fill(null).map((_, i) => {
        return '-';
      });
      const blankTail = 6 - lastDay.getDay();
      const blankHead = 7 - (lastDay.getDate() + blankTail) % 7;
      calendar.unshift(...new Array(blankHead).fill(null));
      calendar.push(...new Array(blankTail).fill(null));
      acc[year][Number(month)] = calendar;
    }
    acc[year][Number(month)][Number(date)] = rawData[dateAsStr].map(log=>`[${log.message}](${path.dirname(log.file)})`).join('<br>');
    return acc;
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
         + [...chunk(month.map((date, i) => date ? `${i}${date !== '-' ? ' 🍺' : ''}<br>${date}` : '-'), 7), '']
          .map(week => ['', ...week].join('|')).join('|\n') + '\n';
      }
      return mdYear;
    }, md);
  }, '');
  console.log(markDown);
})();
