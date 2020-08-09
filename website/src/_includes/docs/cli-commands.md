#### `rome cache dir`

Show the location of the cache directory.

#### `rome cache clear`

Clear all artifacts from the cache directory.

#### `rome check`

Used to find problems in your project. This includes:

 - Dependency verification
 - Formatting
 - Linting
 - `package.json` validation

See [Linting: Command Usage](#command-usage) for more usage information.

**Flags**

- `--apply`

Apply formatting and [safe fixes](#safe-fixes).

- `--changed <branch/commit>`

Only include files that were changed between the specified `branch/commit`. This can be useful for performance in large projects.

If the `branch/commit` is omitted then we default to the default branch, either `main` or `master`. ie. `rome check --changed` is equivalent to `rome check --changed main`.

- `--format-only`

Reformat all files without applying any fixes.

#### `rome config`

Used to modify project configuration. These commands work with all Rome project config locations (see [supported locations](#supported-locations) for more info). When formatting a project config written with [RJSON](#rome-json), comments will be retained.

Before your project config is saved, we will validate it for errors. It is not possible to save an invalid config with `rome config`.

Refer to [Project Configuration: Properties](#properties) for example commands.

#### `rome config enable <key>`

Set the `key` to `true`.

#### `rome config disable <key>`

Set the `key` to `false`.

#### `rome config set <key> <value>`

Set the `key` to a string `value`.

#### `rome config set-directory <key> <value>`

Set the `key` to the string `value`. If `value` is an absolute path then it will be made relative to the config path.

#### `rome config push <key> <value>`

Push the string `value` to an array at `key`. If `key` doesn't exist then it will be created.

#### `rome config location`

Show the config location that would be modified.

#### `rome init`

This command assists in the creation of a new Rome project. Actions that are performed:

 - `rome.rjson` is created that serves as your [project configuration](#project-configuration).
 - `.editorconfig` is created that correctly sets indentation for editors that support [EditorConfig](https://editorconfig.org/).

**Flags**

- `--apply`

Additional operations are applied with this flag:

 - `rome check --apply` is ran which will automatically format and autofix your files.
 - Global variables are extracted from previous errors and automatically added to your project config.

**Uncomitted changes and `--apply`**

Since this command can be destructive and may have unintended consequences, we check if you have any uncomitted changes. It's important to make sure you have everything committed in case you aren't happy with the effects of running this command. ie. you run into a bug, you don't like Rome, or want to try it some other time. You can bypass this restriction by adding the `--allow-dirty` flag.

{% include docs/cli-screenshots/init.md %}

#### `rome logs`

Alias for `rome noop --logs --hang`. See [`--logs` documentation](#--logs) for more info.

This command will never complete.

#### `rome lsp`

Running this command will start a long-running server and communicate via the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) over stdio. This command takes no flags.

#### `rome noop`

This command does nothing. Used in conjunction with other global flags such as [`--logs`](#--logs) and [`--rage`](#--rage).

**Flags**

- `--hang` Instead of immediately exiting, hang the command and never exit unless forced.

#### `rome rage`

Alias for `rome noop --rage`. See [`--rage` documentation](#--rage) for more info.

#### `rome recover`

Whenever Rome needs to write files to the disk, for example when updating the formatting or autofixing a file, we first save a copy of the original file to an internal cache that we call the "recovery store". This is to allow you to revert your changes if necessary. This command is used to interact with this store.

We only keep the content of the last 5 commands that modified files. After that we will delete the oldest entry.

#### `rome recover list`

Show the contents of the recovery store. Including the command that was ran, at what time, files that were changed, and the `recover` commands you can use to perform operations.

{% include docs/cli-screenshots/recover-list.md %}

#### `rome recover pop`

Revert the last command. Equivalent to `rome recover apply <MOST_RECENT_STORE_ID>`.

#### `rome recover apply <id>`

Revert the changes that were made by the corresponding `id`. You can find the `id` by running `rome recover list`.

Running this command will also produce a new store entry with the files that were reverted.

#### `rome recover diff <id>`

Produce a diff of changes between existing files and those included in the `id` store.

#### `rome recover dir`

Print the directory where files are stored.

#### `rome recover clear`

Clear the entire contents of the recovery store.

#### `rome restart`

Equivalent to running [`rome stop`](#rome-stop) and then [`rome start`](#rome-start).

#### `rome start`

Start a [daemon](#daemon), if one isn't already running.

#### `rome status`

Output the status of a running [daemon](#daemon). This includes uptime, file count, and other useful scaling statistics.

#### `rome stop`

Stop a [daemon](#daemon), if one is running.
