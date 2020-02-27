# Getting Started

Rome requires a project configuration in order to operate. This can take three possible forms.

- A `rome.json` file
- A `rome.rjson` file (What is RJSON? See [#13](https://github.com/facebookexperimental/rome/issues/13))
- A `rome` field on `package.json`

This can just be an empty file. It's required in order for Rome to determine all the files in a project. This is important as when running the CLI, we build an in-memory file system listing in order to perform operations like module resolution.

```bash
$ mkdir hello-world
$ cd hello-world
$ echo '{}' >rome.json
```

## Commands

Rome has a dozen different commands. Documented below are some more useful ones when testing functionality.

### `lint`

This command will lint a file with a set of default lints and display the produced diagnostics.

When ran with no arguments, all JavaScript files in a project are linted.

```
$ rome lint file.js
```

### `compile`

This command will compile a file with a set of default transforms. There is currently no options for this command to specify a subset of transforms.

```
$ rome compile file.js
```

### `parse`

This command will parse a file and output a pretty formatted AST.

```
$ rome parse file.js
```

## Daemon

Rome has an optional daemon. When starting the CLI, we'll check if there's a server running, and if there is, we'll connect to and that's where the request will be processed.

You can run the daemon with `rome start`, and stop it with `rome stop`.

The daemon allows Rome to maintain long lived memory caches which can drastically speed up operations. We intend to utilize this server for any LSP integration.

When the CLI is ran without a running server, then we initialize a server inside the CLI process that's only used for the lifetime of the command.
