## CLI

### Global Flags

These are flags that can be added to any Rome command.

##### `--cwd <dir>`

Allows you to explicitly set the current working directory. Otherwise it is the shell cwd when executing the CLI.

##### `--fieri`

Adds some flavor to diagnostics.

##### `--max-diagnostics <num>`

Set the maximum amount of diagnostics that can be displayed before truncation. Defaults to `20`.

##### `--review`

Open an interactive review mode for any diagnostics output from a command.

##### `--show-all-diagnostics`

Output all diagnostics, don't limit to `--max-diagnostics`.

##### `--silent`

Don't write anything to stdout. `stderr` will still be written to for errors. Equivalent to adding `>/dev/null` to a command.

##### `--temporary-daemon`

Spin up the server in a dedicated process. When the command has finished the server will exit. See [Daemon](#daemon) for more information.

##### `--watch`

Some commands support a watch mode that will respond to file changes. See [Commands](#commands) for support.

##### `--verbose-diagnostics`

Output additional information about diagnostics and disable truncation.

### Debugging Flags

These are flags that allow you to debug Rome. These are available in all builds and releases.

##### `--benchmark`

Run a command multiple times and output timing information. The amount of iterations defaults to `10` and can be customized with the `--benchmark-iterations` flag.

This is useful as it will benchmark the command after server initialization and can reuse cache from previous runs making it a realistic information for a user with a server enabled.

##### `--benchmark-iterations <count>`

The amount of iterations to perform when using the `--benchmark` flag. Defaults to `10`.

##### `--logs`

Enables server logs and outputs them to the console.

##### `--log-workers`

Enables worker logs, by default these are not output when running `--logs`.

##### `--log-path <path>`

Instead of logging to the console, write logs to a specific file.

##### `--markers-path <path>`

Collect performance markers.

##### `--profile`

Start CPU profiling all processes and at the end of the command write a CPU profile to disk. Processes include the CLI, server, and workers.

This profile can be loaded into the Chrome Devtools Performance panel.

Upon command completion the profile will be written to the path specified by the `--profile-path` flag.

##### `--profile-path <path>`

Change the path that `--profile` will write to.

Defaults to `Profile-TIMESTAMP.json`.

##### `--profile-sampling <microseconds>`

A sampling CPU profiler, like the one in V8, works by polling on a set interval to track what code is being ran. This means that work which happens very quickly often times will not be captured in a profile.

You can customize this to reduce or increase the timing resolution. The lower the number, the larger but more accurate the profile. However, it may slow down.

Defaults to `200`.

##### `--profile-timeout <milliseconds>`

Write the profile after the specified milliseconds have passed. This is useful for commands that take a long time to run and produce very large profiles.

##### `--no-profile-workers`

Don't include workers in the profile.

##### `--rage`

Produces a rage archive. A rage archive is a `.tar.gz` file that contains information that is useful for debugging performance or bugs. It contains environment and command information, a CPU profile, and logs.

Upon command completion the archive will be written to the path specified by the `--rage-path` flag.

> WARNING: Possible sensitive information such as path names and terminal environment will be visible. It's recommended that you only share this privately with core contributors.

##### `--rage-path <path>`

Change the path that `--rage` will write to.

Defaults to `Rage-TIMESTAMP.tgz`.

##### `--review`

See [reviewing](#reviewing).

##### `--timing`

Output basic timing information on command completion.

##### `--watch`

For commands that support it, rerun and update on file changes.

### Commands

{% include docs/cli-commands.md %}

### Daemon

Rome has a server architecture that's designed to run well as a long-running process, maintaining memory caches and automatically responding to file changes.

This behavior is however **optional**. By default, when running the CLI, we do not create a daemon. However, if there is a daemon available then we will connect to it.

You can explicitly start a daemon with the [`rome start`](#rome-start) command and control it with [`rome restart`](#rome-restart), [`rome status`](#rome-status), and [`rome stop`](#rome-stop).

### Shell Completions

Completions commands are available for `bash` and `fish`. To automatically install them run:

```bash
rome --write-shell-completions bash
rome --write-shell-completions fish
```

This will automatically write the completions to a file and add it to your shell profile if necessary.

> NOTE: This file is static. You may need to run this command whenever Rome is updated for up-to-date completions.

Alternatively you can run:

```bash
rome --log-shell-completions bash
rome --log-shell-completions fish
```

which instead will output the completions to `stdout` rather than a file.

##### `fish`

We will write the completions to `~/.config/fish/completions/rome.fish`. No profile modification is necessary as they are automatically loaded.

##### `bash`

We will write the completions to `~/.rome/rome-completions.sh`. We will add this file as a `source` to either `~/.bashrc` or `~/.bash_profile`, whatever we can find first.
