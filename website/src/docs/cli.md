## CLI

Rome CLI offers some options that can be used with all commands:

```shell
OPTIONS:
    --no-colors      Disable the formatting of markup (print everything as plain text)
```

### Commands 


#### `rome ci`

This command will:

- run the formatter in check mode
- run the linter in check mode


#### `rome init`

This command assists in the creation of a new Rome project. The command will
ask few questions about coding style and configuration extension.

Actions that are performed:

 - `rome.json` is created that serves as your [project configuration](#project-configuration).

The command only works on projects that don't have Rome configuration.