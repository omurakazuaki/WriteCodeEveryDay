import { Command, flags } from '@oclif/command'
import * as fs from 'fs';
import * as path from 'path';
import { Octokit } from '@octokit/rest';
import * as axios from 'axios';
import * as open from 'open';
import { json2html } from '../../../json2html/libs/json2html';
import { auth } from '../auth';

export default class Get extends Command {
  static description = 'get dictionary'

  static examples = [
    `$ dictionary get ascii`,
  ]

  static flags = {
    help: flags.help({char: 'h'})
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
    const json = JSON.stringify(dictionary.data);
    const html = `
    <html>
      <head>
        <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.0.0-beta1/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-giJF6kkoqNQ00vy+HMDP7azOuL0xtbfIcaT9wjKHr8RbDVddVHyTfAAsrekwKmP1" crossorigin="anonymous">
        <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.0.0-beta1/dist/js/bootstrap.bundle.min.js" integrity="sha384-ygbV9kiqUc6oa4msXn9868pTtWMgiQaeYH7/t7LECLbyPA2x65Kgf80OJFdroafW" crossorigin="anonymous"></script>
      <body class='bg-light'>
        <div class="container">
          <div class="py-5">
            <h2>${args.name}</h2>
          </div>
          <div>
            ${json2html(json, 4,
              {
                table: ['table', 'table-striped']
              }
            )}
          </div>
        </div>
      </body>
    </html>`;
    const filepath = path.relative(process.cwd(), path.join(__dirname, '../../', '.temp.html'));
    fs.writeFileSync(filepath, html);
    open(filepath);
  }
}
