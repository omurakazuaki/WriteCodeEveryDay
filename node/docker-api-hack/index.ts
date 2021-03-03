import * as http from 'http';
import * as tar from 'tar';

const request = (options: http.RequestOptions, data?) => {
  const opt: http.RequestOptions = Object.assign({
    socketPath: '/var/run/docker.sock',
    headers: {
      'Content-Type': 'application/json',
      'Host': 'localhost'
    }
  }, options);
  if (data && !data.pipe) {
    opt.headers['Content-Length'] = Buffer.byteLength(data);
  }
  return new Promise<string[]>((resolve, reject) => {
    const buf = [];
    const req = http.request(opt, res => {
      res.on('data', chunk => {
        buf.push(chunk);
      })
      res.on('end', () => {
        resolve(buf);
      })
    });
    req.on('error', e => reject(e));
    if (data?.pipe) {
      data.pipe(req);
      return;
    } else if (data) {
      req.write(data);
    }
    req.end();
  });
};

(async() => {
  try {
    const packed = tar.c({gzip: true}, ['Dockerfile']);
    const tag = 'test';
    const build = await request({method: 'POST', path: `/build?t=${tag}`}, packed);
    console.log(build.toString());

    const images = await request({method: 'GET', path: '/images/json'});
    console.log(images.toString());

    const container = await request({method: 'POST', path: `/containers/create`}, JSON.stringify({
        Image: tag
      })
    );
    console.log(container.toString());

  } catch(e) {
    console.error(e);
  }
})();
