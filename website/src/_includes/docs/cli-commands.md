#### `rome check`

##### Flags

###### `--apply`

###### `--changed <branch/commit>`

Only include files that were changed between the specified `branch/commit`. This can be useful for performance in large projects.

If the `branch/commit` is omitted then we default to the default branch, either `main` or `master`. ie. `rome check --changed` is equivalent to `rome check --changed main`.

#### `rome config`

This command works with all Rome project config locations (see [supported locations](/docs/project-config#supported-locations) for more info). When formatting a project config written with [RJSON](/docs/rjson), comments will be retained.

Before your project config is saved, we will validate it for errors. It is not possible to produce an invalid config with `rome config`.

#### `rome config enable <key>`

Set the `key` to `true`.

#### `rome config disable <key>`

Set the `key` to `false`.

#### `rome config set <key> <value>`

Set the `key` to a string `value`.

#### `rome config set-directory <key> <value>`

Set the `key` to the string `value`. If `value` is an absolute path then it will be made relative to the project base directory.

#### `rome config push <key> <value>`

Push the string `value` to an array at `key`. If `key` doesn't exist then it will be created.


#### `rome init`

This command assists in the creation of a new Rome project. Actions that are performed:

 - `rome.rjson` is created that serves as your [project configuration](/docs/project-config).
 - `.editorconfig` is created that correctly sets indentation for editors that support [EditorConfig](https://editorconfig.org/).
 - `rome check --apply` is ran which will automatically format and autofix your files.
 - Global variables are extracted from previous errors and automatically added to your project config.

##### Uncomitted changes

Since this command can be destructive and may have unintended consequences, we check if you have any uncomitted changes. It's important to make sure you have everything committed in case you aren't happy with the effects of running this command. ie. you run into a bug, you don't like Rome, or want to try it some other time.

You can bypass this restriction by adding the `--allow-dirty` flag.

##### Output

<pre class="language-text"><code class="language-text"><span style="color: CornflowerBlue">$</span> rome init

<strong> Welcome to Rome! Let&apos;s get you started... </strong>

 <strong>Summary</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;"><strong>1</strong></span><span style="color: DodgerBlue;"> </span><span style="color: DodgerBlue;">file</span><span style="color: DodgerBlue;"> saved</span>
  <strong><span style="color: MediumSeaGreen;">✔ </span></strong><span style="color: MediumSeaGreen;">No problems found!</span>

 <strong>Files created</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <span style="opacity: 0.8;">- </span><strong><span style="text-decoration-style: dashed; text-decoration-line: underline;">rome.rjson</span></strong>: Your project configuration. Documentation:
    <a href="https://romefrontend.dev/docs/project-config/">https://romefrontend.dev/docs/project-config/</a>
  <span style="opacity: 0.8;">- </span><strong><span style="text-decoration-style: dashed; text-decoration-line: underline;">.editorconfig</span></strong>: Sets editor formatting and indentation options.
    Documentation: <a href="https://editorconfig.org/">https://editorconfig.org/</a>

 <strong>What next?</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <span style="opacity: 0.8;">1. </span><strong>Setup an editor extension</strong>
     Get live errors as you type and format when you save. Learn more:
     <a href="https://romefrontend.dev/docs/editor-integration/">https://romefrontend.dev/docs/editor-integration/</a>

  <span style="opacity: 0.8;">2. </span><strong>Try a command</strong>
     <i>rome check</i> is used to validate your code, verify formatting, and
     check for lint errors. Run <i>rome --help</i> for a full list of commands
     and flags.

  <span style="opacity: 0.8;">3. </span><strong>Read documentation</strong>
     Our website serves as a comprehensive source of guides and
     documentation <a href="https://romefrontend.dev/">https://romefrontend.dev/</a>

  <span style="opacity: 0.8;">4. </span><strong>Get involved in the community</strong>
     Ask questions, get support, or contribute by participating on
     GitHub (<a href="https://github.com/romefrontend/rome">https://github.com/romefrontend/rome</a>) or our community
     Discord (<a href="https://discord.gg/rome">https://discord.gg/rome</a>)

</pre></code>

#### `rome logs`

Alias for `rome noop --logs --hang`. See [`--logs` documentation](/docs/cli/debugging#--logs) for more info.

This command will never complete.

#### `rome lsp`

Running this command will start a long-running server and communicate via the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) over stdio. This command takes no flags.

#### `rome noop`

This command does nothing. Used in conjunction with other global flags such as [`--logs`](#--logs) and [`--rage`](#--rage).

##### Flags

###### `--hang`

Instead of immediately exiting, hang the command and never exit unless forced.

#### `rome rage`

Alias for `rome noop --rage`. See [`--rage` documentation](/docs/cli/debugging#--rage) for more info.

#### `rome recover`

Whenever Rome needs to write files to the disk, for example when updating the formatting or autofixing a file, we first save a copy of the original file to an internal cache that we call the "recovery store". This is to allow you to revert your changes if necessary. This command is used to interact with this store.

We only keep the content of the last 5 commands that modified files. After that we will delete the oldest entry.

#### `rome recover list`

Show the contents of the recovery store. Including the command that was ran, at what time, files that were changed, and the `recover` commands you can use to perform operations.

**Example output**

<pre class="language-text"><code class="language-text"><span style="color: CornflowerBlue">$</span> rome recover list

<strong> Recovery stores </strong>

 <strong>1595570309210-lint-0</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong>Ran </strong><strong>42m21s</strong><strong> ago</strong> <span style="opacity: 0.8;">(2020-07-24T05:58:29.210Z)</span>"
  <span style="opacity: 0.8;">$ rome lint --apply</span>"

  <span style="opacity: 0.8;">- </span><span style="text-decoration-style: dashed; text-decoration-line: underline;">src/App.ts</span>
  <span style="opacity: 0.8;">- </span><span style="text-decoration-style: dashed; text-decoration-line: underline;">src/UserPage.ts</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">To select specific files to patch run:</span>
  <span style="opacity: 0.8;">$ rome recover apply 1595570309210-lint-0 --select</span>"

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">To see the changes with this patch run:</span>
  <span style="opacity: 0.8;">$ rome recover diff 1595570309210-lint-0</span>"

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">To apply </span><span style="color: DodgerBlue;"><strong>everything</strong></span><span style="color: DodgerBlue;"> in this patch run:</span>
  <span style="opacity: 0.8;">$ rome recover apply 1595570309210-lint-0</span>"

</pre></code>

#### `rome recover pop`

Revert the last command. Equivalent to `rome recover apply <MOST_RECENT_STORE_ID>`.

#### `rome recover apply <id>`

Revert the changes that were made by the corresponding `id`. You can find the `id` by running `rome recover list`.

Running this command will also produce a new store entry with the files that were reverted.

#### `rome recover diff <id>`

Produce a diff of changes between existing files and those included in the `id` store.

#### `rome recover clear`

Clear the entire contents of the recovery store.

#### `rome restart`

Equivalent to running [`rome stop`](/docs/cli/commands/stop) and then [`rome start`](/docs/cli/commands/start).

#### `rome start`

Start a [daemon](#daemon), if one isn't already running.

#### `rome status`

Output the status of a running [daemon](#daemon). This includes uptime, file count, and other useful scaling statistics.

#### `rome stop`

Stop a [daemon](#daemon), if one is running.
