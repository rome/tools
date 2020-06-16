# Usage

## `rome init`

The `init` command helps you to initially setup your project for `rome`. You will be prompted with an interactive wizard that can be followed step by step.

## `rome run`

The `rome run` command will run whatever file is passed to it. Use this command with your project's entry file, for example:

```bash
rome run index.js
```

Keep in mind that Rome is still under active development and may not be able to properly process all source files. If you are able to run a file with `node`, but not with `rome`, please [create an issue](https://github.com/romejs/rome/issues/new?labels=bug&template=01_bug.md&title=).

## `rome lint`

This command will lint your project with a set of default lints and display the produced diagnonstics. For example:

```javascript
rome lint
```

## `rome compile`

This command will compile a file with a set of default transforms. At the moment, we do not support options to specify a subset of transforms.

```bash
rome compile file.js
```

## `rome parse`

This command will parse a file and output a pretty formatted AST.

```bash
rome parse file.js
```