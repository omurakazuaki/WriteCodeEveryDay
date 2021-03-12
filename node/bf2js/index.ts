import * as fs from 'fs';
import * as path from 'path';

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
      steps.push(`${nest()}while (m[p]) {`);
      nestCount++;
    } else if (match?.name  === 'cls') {
      nestCount--;
      steps.push(`${nest()}}`);
    } else if (match?.name  === 'moveValR') {
      const delta = (match.command.length - 4) / 2;
      steps.push(`${nest()}m[p+${delta}]+=m[p];`);
      steps.push(`${nest()}m[p]=0;`);
    } else if (match?.name === 'moveValL') {
      const delta = (match.command.length - 4) / 2;
      steps.push(`${nest()}m[p-${delta}]+=m[p];`);
      steps.push(`${nest()}m[p]=0;`);
    } else if (match?.name === 'clear') {
      steps.push(`${nest()}m[p]=0;`);
    } else if (match?.name === 'moveR') {
      steps.push(`${nest()}p+=${match.command.length};`);
    } else if (match?.name === 'moveL') {
      steps.push(`${nest()}p-=${match.command.length};`);
    } else if (match?.name === 'plus') {
      steps.push(`${nest()}m[p]+=${match.command.length};`);
    } else if (match?.name === 'minus') {
      steps.push(`${nest()}m[p]-=${match.command.length};`);
    } else if (match?.name === 'put') {
      steps.push(`${nest()}write(m[p]);`);
    } else if (match?.name === 'get') {
      steps.push(`${nest()}m[p]=await read();`);
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

const genCode =`// Generate from brainfuck code
const write = char => {
  if (char > -1) {
    process.stdout.write(String.fromCodePoint(char));
  }
};
const read = (() => {
  const buffer = [];
  return () => new Promise((resolve, _) => {
    if (buffer.length) {
      resolve(buffer.shift());
    } else {
      process.stdin
        .resume()
        .setEncoding('utf8')
        .once('data', (chunk) => {
          buffer.push(...Array.from(chunk).map(c=>c.codePointAt(0)));
          resolve(buffer.shift());
          process.stdin.pause();
        })
        .once('end',() => {
          buffer.push(-1);
        });
    }
  });
})();

const m = new Int8Array(30000).fill(0);
let p = 0;

(async() => {
  ${gen(code).join('\n  ')}
})();`;
//eval(genCode);

const binPath = path.join(__dirname, 'bin');
fs.mkdirSync(binPath, { recursive: true });
const srcPath = path.join(binPath, `${programName}.js`);
fs.writeFileSync(srcPath, genCode);
