---
layout: layout.njk
---

# Installation

## Before you continue

To install Rome, you must have `node` and `npm` installed on your system. If you do not have `node` and `npm`
installed, install them before continuing.

## Cloning and building

Rome is not available via `npm` and must be installed from GitHub.
In a folder of your choice, clone the `rome` repository:

```bash
git clone https://github.com/romejs/rome
```

Then, navigate into it and build `rome`:

```bash
cd rome; ./scripts/build-release dist
```

On Windows 10 build `rome` using the following command using PowerShell 7:

```powershell
cd rome && node scripts/build-release dist
```

Now, install `rome` globally:

```
npm install -g ./dist/
```

Congratulations! Rome is installed.

When it comes time to update Rome, repeat the above process. `npm` will
automatically overwrite your existing Rome installation with the new version.

# Getting-started


While Rome seeks to fill the role of many tools in the JavaScript
ecosystem, it can be integrated into existing projects and used
as much or as little as you like.

## Integrating Rome

First, navigate into your project folder:

```bash
cd my_existing_project
```

Now, create a Rome configuration for your project. When prompted,
use the recommended settings:

```bash
rome init
```

## What did we do?

Running `rome init` with the recommended settings creates a Rome
configuration file, `rome.json`, which looks like this:

```json
{
  "version": "^0.0.52",
  "lint": {
    "enabled": true
  }
}
```

This file tells `rome` that it should be at least version 0.0.52
in order to work with your project, and that it should lint your code.
If you want to disable linting or apply advanced settings, see
the `rome init` documentation.

## Running your code

The `rome run` command will run whatever file is passed to
it. Use this command with your project's main file, for example:

```bash
rome run index.js
```

Rome is still under active development and may not be able to properly
process all source files. If you are able to run a file with `node` but
not with `rome`, please [create an issue](https://github.com/romejs/rome/issues/new?labels=bug&template=01_bug.md&title=)

## Other Commands

### `lint`

This command will lint a file with a set of default lints and display the produced diagnostics.
When ran with no arguments, all JavaScript files in a project are linted. For example:

```bash
rome lint file.js
```

### `compile`

This command will compile a file with a set of default transforms. There is currently no options for this command to specify a subset of transforms.

```
rome compile file.js
```

### `parse`

This command will parse a file and output a pretty formatted AST.

```
rome parse file.js
```


## `init`

The `init` command helps you to initially setup your project for `rome`. Inside your project root directory run:

```bash
$ rome init
```

Following this `rome` will ask you whether you want to use the default setup or customize your project further. You can switch between `yes` and `no` with the arrow keys. Press `enter` to confirm your choice.

```bash
  Welcome to Rome!

❯ Use recommended settings?
ℹ Use arrow keys and then enter to select an option
  ◉ Yes
  ◯ No
```

### Default configuration

If you choose `yes`, `rome` will create a default configuration file `rome.json` that looks like this:

```json
{
  "version": "^0.0.52",
  "lint": {
    "enabled": true
  }
}
```

### Customized configuration

If you choose to customize your project further with the `no` option, you will be guided through a set of questions. First you get the chance give your project a name. Enter the name of your project and confirm with the `enter` key.

```bash
 Welcome to Rome!

ℹ Press space to select an option and enter to confirm
❯ Use recommended settings?: No
? Project name:
```

After this you have a choice of whether you want to enable linting, formatting and/or testing. You can move between the choices with the arrow keys. Check and uncheck a box with the `space` key.

```bash
 Welcome to Rome!

❯ Use recommended settings?: No
? Project name: hello-world
❯ Features enabled
ℹ Use arrow keys and space to select or deselect options and then enter to confirm
  ☑ Lint
  ☐ Format
```

Choosing all options results in a config file like this:

```json
{
  "name": "hello-world",
  "version": "^0.0.52",
  "lint": {
    "enabled": true
  },
  "format": {
    "enabled": true
  }
}
```

### I already have a configuration file

If the project already contains a configuration file the `init` command will exit with an error.

```bash
✖ rome.json file already exists
ℹ Use `rome config` to update an existing config
```

Instead of `init` use the `config` command to update your configuration.

# Commands

## `lint`

The `lint` command checks your project files for code problems, such as unused variables. If no arguments are given, the entire project will be included.

```bash
rome lint [files]
```

### Checking only certain files

If you want to lint only certain files, you can add the files you want to check:

```bash
$ rome lint index.js hello.js
```

### Interpreting the result

If `rome` did not detect any problems, you'll get the a result like this:

```bash
ℹ 1 file linted
✔ No known problems!
```

However if there is something not ok, `rome` will give you error messages in the following format:

```bash
foobar.js:1 parse/js ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ import and export can only appear in a module

  > 1 │ import { join } from "path";
      │ ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

  ℹ Change the extension to .mjs to turn this file into a module

  ℹ Add "type": "module" to your package.json
```

These messages contain the following sections:

```
<the affected file>:<line>:<column> <the linter rule that was violated>
-----------------------------------

x description of what is not ok

> 1 | the line of code that is problematic
    |     ^^^ <the part of the line that is not ok>

 ℹ helpful message(s) of what you can do to fix the problem
```

You'll get 1 message for each problem.

### Example

The following code has 2 unused variables: `join` in line 1 and `unused` in line 3.

```js
const {join} = require('path');

const unused = 'I am not used :)';

console.log('hello world!');
```

The result for this code would look like this:

```bash
index.js:1:8 lint/unusedVariables ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Unused variable join

  > 1 │ const { join } = require("path");
      │         ^^^^
    2 │
    3 │ const unused = "I am not used :)";

 index.js:3:6 lint/unusedVariables ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Unused variable unused

    1 │ const { join } = require("path");
    2 │
  > 3 │ const unused = "I am not used :)";
      │       ^^^^^^
    4 │
    5 │ console.log("hello world!");

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
ℹ 1 file linted
✖ Found 2 problems
```

# Contributing

We can use help in a bunch of areas and any help is appreciated. Our [GitHub issues](https://github.com/romejs/rome/issues) serve as a place for any discussion, whether it's bug reports, questions, project direction etc. As the project grows this policy may change.

Our [Discord server](https://discord.gg/9WxHa5d) is open for help and more adhoc discussion. All activity on the Discord is still moderated and will be strictly enforced under the project's [Code of Conduct](https://github.com/romejs/rome/blob/master/.github/CODE_OF_CONDUCT.md).

## Getting Started

Getting started with developing Rome is as easy as three commands. You will need Node v12 or above.

```bash
git clone https://github.com/romejs/rome
cd rome
scripts/dev-rome --help
```

> Note: If you previously ran the user-facing [installation instructions](../introduction/installation), the `dist` directory must be deleted before running any development commands.

No dependency installation step is required as we check in our `node_modules` folder that contains only a copy of TypeScript and some definitions.

Refer to [Getting Started](../introduction/getting-started.md) for more usage documentation.

## Testing

You can run the test suite with the following command:

```bash
scripts/dev-rome test
```

This will run all tests inside of any `__rtests__` directories.

## Type Checking

Run TypeScript with code emitting disabled to perform a full typecheck outside the editor.

```bash
node_modules/.bin/tsc --noEmit
```
