import { buildSchema } from 'graphql';
import * as fs from 'fs';
import * as path from 'path';

export const loadSchema = name => {
  return buildSchema(fs.readFileSync(path.join(__dirname, `${name}.graphql`)).toString());
}
