const Git = require('../libs/git');

(async() => {
  const git = new Git();
  const commit = await git.commitObject();
  console.log(JSON.stringify(commit, null, ' '));

  const tree = await git.object(commit.content.tree);
  console.log(JSON.stringify(tree, null, ' '));

  const blob = await git.object(tree.content[0].hash);
  console.log(JSON.stringify(blob, null, ' '));

  const history = await git.commitHistory();
  console.log(JSON.stringify(history, null, ' '));

  const gitJsBlob = await git.findObject('node/libs/git.js');
  console.log(JSON.stringify(gitJsBlob, null, ' '));

})();
