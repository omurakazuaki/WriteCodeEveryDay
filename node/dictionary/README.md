dictionary
==========



[![oclif](https://img.shields.io/badge/cli-oclif-brightgreen.svg)](https://oclif.io)
[![Version](https://img.shields.io/npm/v/dictionary.svg)](https://npmjs.org/package/dictionary)
[![Downloads/week](https://img.shields.io/npm/dw/dictionary.svg)](https://npmjs.org/package/dictionary)
[![License](https://img.shields.io/npm/l/dictionary.svg)](https://github.com/omurakazuaki/dictionary/blob/master/package.json)

<!-- toc -->
* [Usage](#usage)
* [Commands](#commands)
<!-- tocstop -->
# Usage
<!-- usage -->
```sh-session
$ npm install -g dictionary
$ dictionary COMMAND
running command...
$ dictionary (-v|--version|version)
dictionary/0.0.0 linux-x64 node-v13.13.0
$ dictionary --help [COMMAND]
USAGE
  $ dictionary COMMAND
...
```
<!-- usagestop -->
# Commands
<!-- commands -->
* [`dictionary add FILEPATH`](#dictionary-add-filepath)
* [`dictionary get NAME`](#dictionary-get-name)
* [`dictionary help [COMMAND]`](#dictionary-help-command)
* [`dictionary list`](#dictionary-list)

## `dictionary add FILEPATH`

Add dictionary

```
USAGE
  $ dictionary add FILEPATH

OPTIONS
  -h, --help  show CLI help

EXAMPLE
  $ dictionary add ascii.json
```

_See code: [src/commands/add.ts](https://github.com/omurakazuaki/dictionary/blob/v0.0.0/src/commands/add.ts)_

## `dictionary get NAME`

get dictionary

```
USAGE
  $ dictionary get NAME

OPTIONS
  -f, --format
  -h, --help    show CLI help

EXAMPLE
  $ dictionary get ascii
```

_See code: [src/commands/get.ts](https://github.com/omurakazuaki/dictionary/blob/v0.0.0/src/commands/get.ts)_

## `dictionary help [COMMAND]`

display help for dictionary

```
USAGE
  $ dictionary help [COMMAND]

ARGUMENTS
  COMMAND  command to show help for

OPTIONS
  --all  see all commands in CLI
```

_See code: [@oclif/plugin-help](https://github.com/oclif/plugin-help/blob/v3.2.2/src/commands/help.ts)_

## `dictionary list`

list dictionary

```
USAGE
  $ dictionary list

OPTIONS
  -h, --help  show CLI help

EXAMPLE
  $ dictionary list
```

_See code: [src/commands/list.ts](https://github.com/omurakazuaki/dictionary/blob/v0.0.0/src/commands/list.ts)_
<!-- commandsstop -->
