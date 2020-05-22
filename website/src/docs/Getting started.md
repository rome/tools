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
