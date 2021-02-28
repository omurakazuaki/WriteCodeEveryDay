import * as fs from 'fs';
import * as path from 'path';
import { Command, flags } from '@oclif/command';
import { Octokit } from '@octokit/rest';
import { auth } from '../auth';

export default class Add extends Command {
  static description = 'Add dictionary'

  static examples = [
    `$ dictionary add ascii.json`,
  ]

  static flags = {
    help: flags.help({char: 'h'}),
  }

  static args = [
    {name: 'filepath', required: true}
  ]

  async run() {
    const {args, flags} = this.parse(Add);
    const content = fs.readFileSync(args.filepath).toString();
    JSON.parse(content);
    const filename = path.basename(args.filepath);
    const octokit = new Octokit({auth});
    const gists = await octokit.gists.list();
    const gist = gists.data
      .find(dict => dict.description === 'dictionary');
    if (!gist) {
      throw new Error(`Dictionary repository was not found`);
    }
    const files: any = {};
    files[filename] = { content };
    await octokit.gists.update({
      gist_id: gist?.id,
      files
    });

  }
}
