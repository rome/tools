## Reference

### CLI

#### Commands

##### `rome init`

Helps you to set up Rome for a new project by guiding you through the creation of a new `rome.json` [configuration](#configuration) file.

The command fails if the project already has a `rome.json` configuration file.

##### `rome version`

Prints the version of the CLI and whether there's a server (daemon) connected

##### `rome rage`

Prints information for debugging purpose

##### `rome lsp-proxy`

It starts a server for the LSP protocol, which communicates over `stdin` and `stdout`.

This command is useful to interact with the Rome server when developing editors/IDEs. 

##### `rome format`

Runs the formatter on a set of files.

##### `rome check`

Runs the linter on a set of files and reports errors and warnings to the console.

##### `rome ci`

Runs the linter and verifies the formatting of a set of files. It reports errors to the console. If any errors are found the process exits with a code of `1`.

This command is intended to be used in CI workflows.

##### `rome start`

Start the Rome [daemon](#daemon) server

##### `rome stop`

Stop the Rome [daemon](#deamon) server

#### Common Options

##### `--no-colors`

Disable the formatting of markup (print everything as plain text)

##### `--use-server`

Connect to a running instance of the Rome daemon server

##### `--files-max-size`

The maximum allowed size for source code files in bytes.

> Default: 1024

#### Global Options

Use these flags to get information about the Rome CLI.

##### `--version`

Prints the Rome version and exits.

##### `--help`

Prints the help message and exits.

### `rome.json`

#### Files

##### `files.maxSize`

The maximum allowed size for source code files in bytes. Files above
this limit will be ignored for performance reason. 

> Default: 1024

#### Linter

##### `linter.enabled`

Enables Rome's linter

> Default: `true`


##### `linter.ignore`

An array of Unix shell style patterns.

```json
{
  "linter": {
    "ignore": ["scripts/*.js"]
  }
}
```

##### `linter.rules.recommended`

Enables the [recommended rules](/docs/lint/rules) for all categories.

> Default: `true`

##### `linter.rules.[category]`

Options that influence the rules of a single category. Rome supports the following categories:

{% include docs/reference/groups.md %}

##### `linter.rules.[category].recommended`

Enables the recommended rules for a single category.

Example:

```json
{
  "linter": {
    "enabled": true,
    "rules": {
      "nursery": {
        "recommended": true
      }
    }
  }
}
```

#### Formatter

##### `formatter.enabled`

Enables Rome's formatter

> Default: `true`

##### `formatter.ignore`

An array of Unix shell style patterns.

```json
{
  "formatter": {
    "ignore": ["scripts/*.js"]
  }
}
```

##### `formatter.indentStyle`

The style of the indentation. It can be `"tab"` or `"space"`.

> Default: `tab`

Rome's default is `"tab"`.

##### `formatter.indentSize`

How big the indentation should be.

##### `formatter.lineWidth`

How many characters can be written on a single line.

> Default: `80`

#### JavaScript

##### `javascript.formatter.quoteStyle`

The type of quote used when representing string literals. It can be `single` or `double`.

> Default: `double`

##### `javascript.formatter.quoteProperties`

When properties inside objects should be quoted. It can be `asNeeded` or `preserve`.

> Default: `asNeeded`
 

##### `javascript.formatter.trailingComma`

Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Possible values:
- `all`, the trailing comma is always added
- `es5`, the trailing comma is added only in places where it's supported by older version of JavaScript
- `none`, trailing commas are never added

> Default: `all`
