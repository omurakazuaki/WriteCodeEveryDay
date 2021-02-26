# MarkDown Calendar

## Create the git instance
```js
const git = new Git();
```

## Read the commit object pointed to by HEAD

```js
const commit = git.commitObject();
```

```json
{
 "hash": "09c6e01b074b3d8ceaa7800c326c6949a47edc97",
 "type": "commit",
 "content": {
  "tree": "c39de4c0a26e9abddf83625e6503debfa9508b25",
  "parent": "fe3d4064a1f5fc326c71f2390734002a98b56a2c",
  "author": {
   "name": "omurakazuaki",
   "email": "omura@enrapt.jp",
   "date": "2021-02-25T23:21:51.000Z"
  },
  "committer": {
   "name": "omurakazuaki",
   "email": "omura@enrapt.jp",
   "date": "2021-02-25T23:55:34.000Z"
  },
  "message": "refactor path2hub"
 }
}
```
## Read the tree or blob object

### tree

```js
const tree = await git.object(commit.content.tree);
```

```json
{
 "hash": "c39de4c0a26e9abddf83625e6503debfa9508b25",
 "type": "tree",
 "content": [
  {
   "mode": "100644",
   "name": ".gitignore",
   "hash": "f46e1dca287183eb4b5f42bcde1a80387e7de09a"
  },
  {
   "mode": "040000",
   "name": ".husky",
   "hash": "abc799809fae97c9dc76895d48f4b30137f8b6f5"
  },
  {
   "mode": "100644",
   "name": "README.md",
   "hash": "a6749a736506cc232dd949a3f9ebaf471401977e"
  },
  {
   "mode": "040000",
   "name": "bin",
   "hash": "8834293513a2f53c63badb1fa0686c7b5e76f4d3"
  },
  {
   "mode": "040000",
   "name": "node",
   "hash": "fd4f11b5509174a3e4832dc86bedae486d0bd8fe"
  },
  {
   "mode": "100644",
   "name": "package-lock.json",
   "hash": "651c9c909d35e4588b05cda9195c12ed65fbb564"
  },
  {
   "mode": "100644",
   "name": "package.json",
   "hash": "ae9974a54ebf0585bd85ff3985a491c364f51c09"
  }
 ]
}
```

### blob

```js
const blob = await git.object(tree.content[0].hash);
```

```json
{
 "hash": "f46e1dca287183eb4b5f42bcde1a80387e7de09a",
 "type": "blob",
 "content": "node_modules\nidea.md\n"
}
```
