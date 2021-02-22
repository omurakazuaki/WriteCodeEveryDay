import { stdin } from 'process';
import * as readLine from 'readline';
import { preProcessFile } from 'typescript';

type Write = (char: number) => void;
type Read = () => Promise<number>;

type Option = {
  write?: Write;
  read?: Read;
  memorySize?: number;
  cellBits?: 8 | 16 | 32;
  commands?: {
    inc: string;
    dec: string;
    nxt: string;
    prv: string;
    put: string;
    get: string;
    opn: string;
    cls: string;
  }
};

export class Brainfuck {

  opt: Option;
  memoryConstructor: Int8ArrayConstructor|Int16ArrayConstructor|Int32ArrayConstructor;

  constructor(opt?: Option) {
    const defaultOption: Option = {
      write: char => {
        if (char > -1) {
          process.stdout.write(String.fromCodePoint(char));
        }
      },
      read: (() => {
        const buffer = [];
        return () => new Promise<number>((resolve, _) => {
          if (buffer.length) {
            resolve(buffer.shift());
          } else {
            process.stdin
              .resume()
              .setEncoding('utf8')
              .once('data', (chunk: string) => {
                buffer.push(...Array.from(chunk).map(c=>c.codePointAt(0)));
                resolve(buffer.shift());
                process.stdin.pause();
              })
              .once('end',() => {
                buffer.push(-1);
              });
          }
        });
      })(),
      memorySize: 1024 * 8,
      cellBits: 8,
      commands: {
        inc: '+',
        dec: '-',
        nxt: '>',
        prv: '<',
        put: '.',
        get: ',',
        opn: '[',
        cls: ']',
      }
    }
    this.opt = Object.assign(defaultOption, opt);
    this.memoryConstructor = {
      '8': Int8Array,
      '16': Int16Array,
      '32': Int32Array
    }[this.opt.cellBits||'8'];
  }

  async execute(code: string): Promise<Int8Array|Int16Array|Int32Array> {
    const memory: Int8Array|Int16Array|Int32Array = new this.memoryConstructor(this.opt.memorySize).fill(0);
    const jumpPoints: number[] = [], skipPoints: number[] = [];
    const commands = Object.values(this.opt.commands);
    for (let ptr = 0, i = 0; code[i];) {
      const currentCode = code.slice(i);
      const command = commands.find(c => currentCode.indexOf(c) === 0);
      if (command === this.opt.commands.opn) {
        if (memory[ptr] !== 0) {
          jumpPoints.push(i);
        } else {
          skipPoints.push(i);
        }
      } else if (command === this.opt.commands.cls) {
        if (!jumpPoints.length && !skipPoints.length) {
          throw Error(`Syntax error: ${this.opt.commands.opn} expected.(index: ${i})`);
        } else if (!skipPoints.length) {
          i = jumpPoints.pop();
          continue;
        } else {
          skipPoints.pop();
        }
      } else if (skipPoints.length) {
        // do nothing
      } else if (command === this.opt.commands.nxt) {
        ptr = ptr < this.opt.memorySize - 1 ? ptr + 1 : 0;
      } else if (command === this.opt.commands.prv) {
        ptr = ptr > 0 ? ptr - 1 : this.opt.memorySize - 1;
      } else if (command === this.opt.commands.inc) {
        memory[ptr]++;
      } else if (command === this.opt.commands.dec) {
        memory[ptr]--;
      } else if (command === this.opt.commands.put) {
        this.opt.write(memory[ptr]);
      } else if (command === this.opt.commands.get) {
        memory[ptr] = await this.opt.read() || 0;
      } else {
        // do nothing
      }
      i += command?.length || 1;
    }
    if (jumpPoints.length || skipPoints.length) {
      throw Error(`Syntax error: ${this.opt.commands.cls} expected.(index: ${(jumpPoints.pop() || skipPoints.pop())})`);
    }
    return memory;
  }
};
