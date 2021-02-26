const Git = require('../libs/git');

(async() => {
  const git = new Git();
  const commit = await git.commitObject();
  console.log(commit);

  const tree = await git.object(commit.content.tree);
  console.log(tree);

  const blob = await git.object(tree.content[0].hash);
  console.log(blob);

  const history = await git.commitHistory();
  console.log(history);

  const gitJsBlob = await git.findObject('node/libs/git.js');
  console.log(gitJsBlob);

  console.log(git.toHash(gitJsBlob) === gitJsBlob.hash);
  console.log(git.toHash(tree) === tree.hash);
  console.log(git.toHash(commit) === commit.hash);

})();
