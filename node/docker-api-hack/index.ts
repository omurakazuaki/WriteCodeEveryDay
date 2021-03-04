import * as http from 'http';
import { Readable } from 'stream';
import * as tar from 'tar';
import * as path from 'path';

const request = (options: http.RequestOptions, data?: Readable | string) => {
  const opt: http.RequestOptions = Object.assign({
    socketPath: '/var/run/docker.sock',
    headers: {
      'Content-Type': 'application/json',
      'Host': 'localhost'
    }
  }, options);
  if (typeof data === 'string') {
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
    if (typeof data === 'string') {
      req.write(data);
    } else if (data) {
      data.pipe(req);
      return;
    }
    req.end();
  });
};

(async() => {
  try {
    const packed = tar.c({gzip: true, cwd: __dirname}, ['Dockerfile']);
    const tag = 'test';
    const build = await request({method: 'POST', path: `/build?t=${tag}`}, packed);
    console.log(build.toString());

    const images = await request({method: 'GET', path: '/images/json'});
    console.log(images.toString());

    const containerName = 'test_container';
    const container = await request({method: 'POST', path: `/containers/create?name=${containerName}`}, JSON.stringify({
        Image: tag,
        Cmd: ['bf', '/bf/hello.bf'],
        HostConfig: {
          Mounts: [
            {
              Type: 'bind',
              Source: path.resolve(path.join(__dirname, '../brainfuck/bf')),
              Target: '/bf',
              ReadOnly: true
            }
          ]
        }
      })
    );
    console.log(container.toString());

    const start = await request({method: 'POST', path: `/containers/${containerName}/start`});
    console.log(start.toString());

    const logs = await request({method: 'GET', path: `/containers/${containerName}/logs?stdout="true"`});
    console.log(logs.toString());

    const remove = await request({method: 'DELETE', path: `/containers/${containerName}`});
    console.log(remove.toString());
  } catch(e) {
    console.error(e);
  }
})();
