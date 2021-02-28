import { Command, flags } from '@oclif/command'
import { Octokit } from '@octokit/rest';
import * as axios from 'axios';
import { auth } from '../auth';

export default class Get extends Command {
  static description = 'get dictionary'

  static examples = [
    `$ dictionary get ascii`,
  ]

  static flags = {
    help: flags.help({char: 'h'}),
    format: flags.boolean({char: 'f'}),
  }

  static args = [{name: 'name', required: true}]

  async run() {
    const {args, flags} = this.parse(Get);
    const octokit = new Octokit({auth});
    const gists = await octokit.gists.list();
    const gistFile = gists.data
      .find(dict => dict.description === 'dictionary')?.files[`${args.name}.json`];
    if (!gistFile?.raw_url) {
      throw new Error(`Dictionary was not found: ${args.name}`);
    }
    const dictionary = await axios.default.get(gistFile?.raw_url);
    console.log(JSON.stringify(dictionary.data, null, ' '));
  }
}
