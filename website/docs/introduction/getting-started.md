---
id: getting-started
title: Getting Started
sidebar_label: Getting Started
---

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
not with `rome`, please [create an issue](https://github.com/facebookexperimental/rome/issues/new?labels=bug&template=01_bug.md&title=)

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
