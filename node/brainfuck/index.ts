import * as fs from 'fs';
import { Brainfuck } from './libs/Brainfuck';


(async () => {
  const args = process.argv.slice(2);
  const sourceFile = args.shift();
  const source = fs.readFileSync(sourceFile, 'utf-8');
  const bf = new Brainfuck();
  await bf.execute(source);
})();
