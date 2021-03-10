import * as fs from 'fs';
import * as path from 'path';

let bfCode = "";

const move = (offset: number) => {
  const absOffset = Math.abs(offset);
  const op = offset > 0 ? '>' : '<';
    bfCode += op.repeat(absOffset);
}

const add = (val: number) => {
  if (val === 0) return;
  const absVal = Math.abs(val);
  const op = val > 0 ? '+' : '-';
  const sizes: number[] = [1000];
  for (let i = 1; i <= absVal; i++) {
    const x = absVal / i | 0;
    const y = absVal % i;
    sizes.push(i + x + y + (i == 1 ? -1 : 9));
  }
  const min = sizes.slice().sort((a,b)=>a-b)[0];
  const n = sizes.findIndex(x => x==min);
  if (n == 1) {
    bfCode += op.repeat(absVal);
  } else {
    const x = absVal / n | 0;
    const y = val % n;
    move(1);
    add(n);
    bfCode += '[';
    move(-1);
    bfCode += op.repeat(x);
    move(1);
    add(-1);
    bfCode += ']';
    move(-1);
    add(y);
  }
}

const reset = () => {
  bfCode += '[-]';
}

const put = () => {
  bfCode += '.';
}

const get = () => {
  bfCode += ',';
}

const echo = () => {
  get();
  add(1);
  bfCode += '[';
  add(-1);
  put();
  get();
  add(1);
  bfCode += ']';
}

const printString = (val: String) => {
  val.split('')
    .map(v=>v.charCodeAt(0))
    .map((v, i, a) => i === 0 ? v : v - a[i-1])
    .forEach(v => {
      add(v);
      put();
  });
}

printString(`Hello, World!
Hello, `);
echo();

const binPath = path.join(__dirname, 'bin');
fs.mkdirSync(binPath, { recursive: true });
const srcPath = path.join(binPath, `gen.bf`);
fs.writeFileSync(srcPath, bfCode);

