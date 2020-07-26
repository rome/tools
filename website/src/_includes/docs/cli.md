## CLI

### Commands

- [`check`](/docs/cli/commands/check)
- [`config`](/docs/cli/commands/config)
- [`init`](/docs/cli/commands/init)
- [`logs`](/docs/cli/commands/logs)
- [`lsp`](/docs/cli/commands/lsp)
- [`noop`](/docs/cli/commands/noop)
- [`rage`](/docs/cli/commands/rage)
- [`recover`](/docs/cli/commands/recover)
- [`start`](/docs/cli/commands/start)
- [`status`](/docs/cli/commands/status)
- [`stop`](/docs/cli/commands/stop)

### Global Flags

These are flags that can be added to any Rome command.

##### `--cwd \<dir>`

Allows you to explicitly set the current working directory. Otherwise it is the shell cwd when executing the CLI.

##### `--fieri`

Adds some flavor to diagnostics.

##### `--max-diagnostics \<num>`

##### `--show-all-diagnostics`

##### `--silent`

##### `--temporary-daemon`

##### `--verbose-diagnostics`

### Debugging Flags

These are flags that allow you to debug Rome. These are available in any build and release, including production.

##### `--benchmark`

Run a command multiple times and output timing information. The amount of iterations defaults to `10` and can be customized with the `--benchmark-iterations` flag.

This is especially useful as it will benchmark the command after server initialization and can reuse cache from previous runs making it a realistic information for a user with a server enabled.

##### `--benchmark-iterations \<count>`

The amount of iterations to perform when using the `--benchmark` flag. Defaults to `10`.

##### `--logs`

Enables server logs and outputs them to the console.

##### `--log-workers`

Enables worker logs, by default these are not output when running `--logs`.

##### `--log-path \<path>`

Instead of logging to the console, write logs to a specific file.

##### `--markers-path \<path>`

Collect performance markers.

##### `--profile`

Start CPU profiling all processes and at the end of the command write a CPU profile to disk. Processes include the CLI, server, and workers.

This profile can be loaded into the Chrome Devtools Performance panel.

By default the profile is written to `Profile-TIMESTAMP.json` and can be customized with the `--profile-path` flag.

##### `--profile-path \<path>`

Change the path that `--profile` will write to.

##### `--profile-sampling \<microsec>`

The way the CPU profiler works is that every `microsec` it will check what code is being ran and use it in the profile. This means that work that happens very quickly often times wont be captured in a profile. You can customize this to reduce or increase the profile resolution. Defaults to `200`.

##### `--profile-timeout \<millisec>`

Write the profile after `millisec` have passed. This is useful for commands that take a long time to run and produce very large profiles.

##### `--no-profile-workers`

Don't include workers in the profile.

##### `--rage`

This flag is used to produce a rage archive. A rage archive is a `.tar.gz` file that contains information that is useful for debugging performance or bugs. It contains environment information, a CPU profile, logs, and the command.

##### `--rage-path \<path>`

Change the path that `--rage` will write to.

##### `--timing`

Output basic timing information on command completion.

### Shell Autocomplete

```bash
rome --generate-autocomplete bash
```


### Server

Rome has a server architecture designed to function best as a long-running process, maintaining memory caches and automatically responding to file changes.

However this is not the default behaviour of Rome. When running a CLI command, we first check if there's a running server, and if so, dispatch the request to it. If there is no server running then we only create a server inside the CLI, and only for the lifetime of the command.

 - [`rome restart`](/docs/cli/commands/restart)
 - [`rome start`](/docs/cli/commands/start)
 - [`rome status`](/docs/cli/commands/status)
 - [`rome stop`](/docs/cli/commands/stop)
