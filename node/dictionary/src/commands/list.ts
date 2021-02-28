import { Command, flags } from '@oclif/command'
import { Octokit } from '@octokit/rest';
import { auth } from '../auth';

export default class List extends Command {
  static description = 'list dictionary'

  static examples = [
    `$ dictionary list`,
  ]

  static flags = {
    help: flags.help({char: 'h'}),
  }

  async run() {
    this.parse(List)
    const octokit = new Octokit({auth});
    const gists = await octokit.gists.list();
    const gistFiles = gists.data
      .find(dict => dict.description === 'dictionary')?.files;
    if (!gistFiles) {
      throw new Error(`Dictionary repository was not found`);
    }
    console.log(Object.keys(gistFiles).map(f=>f.replace(/\..+$/, '')).join('\n'));
  }
}
