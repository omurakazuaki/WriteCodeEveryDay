import * as Mustache from 'mustache';

export const json2html = (json: string, rowDataNum: number=1, classesMapping: any={}) => {
  const data: object[] = JSON.parse(json);
  const columns = data
    .map(cols=>Object.keys(cols))
    .flat()
    .filter((col, i, self) => self.indexOf(col) === i);
  const classes = (tagName: string) => classesMapping[tagName]?.join(' ') || '';

  const chunk = ([...array], size = 1) => {
    return array.reduce((acc, _, index) => index % size ? acc : [...acc, array.slice(index, index + size)], []);
  }

  const chunkedData = chunk(data, rowDataNum);

  const template = `
    <table class="${classes('table')}" cellspacing="0" width="100%">
      <thead class="${classes('thead')}">
        <tr class="${classes('tr')}">
          ${columns.map(col=>`<th>${col}</th>`).join('\n').repeat(rowDataNum)}
        </tr>
      </thead>
      <tbody class="${classes('tbody')}">
        {{#chunkedData}}
          <tr class="${classes('tr')}">
          {{#.}}
            ${columns.map(col=>`<td>{{${col}}}</td>`).join('\n')}
          {{/.}}
          </tr>
        {{/chunkedData}}
      </tbody>
    </table>`;
  return Mustache.render(template, {chunkedData});
}
