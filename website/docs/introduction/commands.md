---
id: commands
title: Commands
sidebar_label: Commands
description: Detailed description of the available commands.
---

## Commands

### `init`

The `init` command helps you to initially setup your project for `rome`. Inside your project root directory run:

```bash
$ rome init
```

Following this `rome` will ask you whether you want to use the default setup or customize your project further. You can switch between `yes` and `no` with the arrow keys. Press `enter` to confirm your choice.

```bash
 Welcome to Rome!

ℹ Press space to select an option and enter to confirm
❯ Use recommended settings?
  ◉ Yes
  ◯ No
```

#### Default configuration

If you choose `yes`, `rome` will create a default configuration file `rome.json` that looks like this:

```json
{
  "version": "^0.0.52",
  "lint": {
    "enabled": true
  }
}
```

#### Customized configuration

If you choose to customize your project further with the `no` option, you will be guided through a set of questions. First you get the chance give your project a name. Enter the name of your project and confirm with the `enter` key.

```bash
 Welcome to Rome!

ℹ Press space to select an option and enter to confirm
❯ Use recommended settings?: No
? Project name:
```

After this you have a choice of whether you want to enable linting and formatting. You can move between the choices with the arrow keys. Check and uncheck a box with the `space` key.

```bash
 Welcome to Rome!

ℹ Press space to select an option and enter to confirm
❯ Use recommended settings?: No
? Project name: hello-world
❯ Features enabled
  ☑ Lint
  ☐ Format
```

Choosing both results in a config file like this:

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

### `lint`

The `lint` command checks your project files for code problems, such as unused variables. If no arguments are given, the entire project will be included.

```bash
rome lint [files]
```

#### Checking only certain files

If you want to lint only certain files, you can add the files you want to check:

```bash
$ rome lint index.js hello.js
```

#### Interpreting the result

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

#### Example

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
