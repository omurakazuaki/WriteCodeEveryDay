module.exports = () => {
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
