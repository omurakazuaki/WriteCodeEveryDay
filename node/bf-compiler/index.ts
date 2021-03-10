import * as fs from 'fs';
import * as path from 'path';
import { exec } from 'child_process';

const args = process.argv.slice(2);
const sourceFile = args.shift();
const code = fs.readFileSync(sourceFile, 'utf-8');
const programName = path.basename(sourceFile, '.bf');

type Command = {name: string, command: string};

type CommandMatcher = (code: string) => Command;

const defaultMatcher = (name: string, regexp: RegExp): CommandMatcher => {
  return (code): Command => {
    const match = code.match(regexp);
    if (match) {
      return {
        name,
        command: match[0]
      }
    }
    return undefined;
  };
}

const commandMatchers: CommandMatcher[] = [
  code => {
    const match = code.match(/^\[-(>+)\+(<+)\]/);
    if (match && match[1].length === match[2].length) {
      return {
        name: 'moveValR',
        command: match[0]
      }
    }
    return undefined
  },
  code => {
    const match = code.match(/^\[-(<+)\+(>+)\]/);
    if (match && match[1].length === match[2].length) {
      return {
        name: 'moveValL',
        command: match[0]
      }
    }
    return undefined
  },
  defaultMatcher('clear', /^\[-\]/),
  defaultMatcher('moveR', /^>+/),
  defaultMatcher('moveL', /^<+/),
  defaultMatcher('plus', /^\++/),
  defaultMatcher('minus', /^\-+/),
  defaultMatcher('opn', /^\[/),
  defaultMatcher('cls', /^\]/),
  defaultMatcher('get', /^,/),
  defaultMatcher('put', /^\./),
];

const gen = (code: string): string[] => {
  const oneLinerCode = code.replace(/\n/g, '');
  const steps: string[] = [];
  let nestCount = 0;
  for (let i = 0; oneLinerCode[i];) {
    const currentCode = oneLinerCode.slice(i);
    const match = commandMatchers.map(matcher => matcher(currentCode)).find(m=>m);
    const nest = () => '    '.repeat(nestCount);
    if (match?.name === 'opn') {
      steps.push(`${nest()}while memory.get() != 0 {`);
      nestCount++;
    } else if (match?.name  === 'cls') {
      nestCount--;
      steps.push(`${nest()}}`);
    } else if (match?.name  === 'moveValR') {
      steps.push(`${nest()}memory.mvv(${(match.command.length - 4) / 2});`);
    } else if (match?.name === 'moveValL') {
      steps.push(`${nest()}memory.mvv(-${(match.command.length - 4) / 2});`);
    } else if (match?.name === 'clear') {
      steps.push(`${nest()}memory.clr();`);
    } else if (match?.name === 'moveR') {
      steps.push(`${nest()}memory.mov(${match.command.length});`);
    } else if (match?.name === 'moveL') {
      steps.push(`${nest()}memory.mov(${-match.command.length});`);
    } else if (match?.name === 'plus') {
      steps.push(`${nest()}memory.add(${match.command.length});`);
    } else if (match?.name === 'minus') {
      steps.push(`${nest()}memory.add(${-match.command.length});`);
    } else if (match?.name === 'put') {
      steps.push(`${nest()}write(memory.get());`);
    } else if (match?.name === 'get') {
      steps.push(`${nest()}memory.put(read());`);
    } else if ( currentCode[0] != '\n') {
      const lastStep = steps.pop();
      if (lastStep && lastStep.trim().startsWith('//')) {
        steps.push(lastStep + currentCode[0]);
      } else {
        steps.push(lastStep);
        steps.push(`${nest()}// ${currentCode[0]}`)
      }
    }
    i += match?.command?.length || 1;
  }
  return steps;
}

const memoryCode = fs.readFileSync(path.join(__dirname,'../../rust/optimaized-bf/src/memory.rs'), 'utf-8');

const rustCode =`// Generate from brainfuck code
use std::io::{self, Read};

${memoryCode}

fn write(c: i8) {
    if c > -1 {
        print!("{}", c as u8 as char);
    }
}

fn stdin_reader() -> impl FnMut() -> isize {
    let mut buff: Vec<isize> = vec![];
    move || -> isize {
        if buff.is_empty() {
            for byte in io::stdin().bytes() {
                match byte {
                    Ok(b) => buff.push(b as isize),
                    Err(_) => buff.push(-1)
                }
            }
        }
        if buff.is_empty() {
            return -1;
        } else {
            return buff.drain(0..1).as_slice()[0];
        }
    }
}

fn main() {
    let mut memory: Memory = Memory::new();
    let mut read = stdin_reader();
    ${gen(code).join('\n    ')}
}`;

const binPath = path.join(__dirname, 'bin');
fs.mkdirSync(binPath, { recursive: true });
const srcPath = path.join(binPath, `${programName}.rs`);
fs.writeFileSync(srcPath, rustCode);
exec(`rustc ${srcPath} -O --out-dir ${binPath}`, (err, stdout, stderr) => {
  if (err) {
    console.error(stderr);
    throw err;
  }
  console.log(stdout);
});
