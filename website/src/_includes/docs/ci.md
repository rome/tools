## CI

Rome's `ci` is a command specifically made to be run in continuous integrations (CI) environments.

Diagnostics emitted by this command are considered errors, and printed to `stderr`. 

This command runs the following:
- formatter, in check mode
- linter

You can use the `--help`

```shell
rome ci --help
```

And know which options are supported:

```shell
Rome CI: Run the linter and formatter check on a set of files

USAGE:
    rome ci [OPTIONS] <INPUTS...>

    INPUTS can be one or more filesystem path, each pointing to a single file or an entire directory to be searched recursively for supported files

OPTIONS:
    --formatter-enabled                      Allow to enable or disable the formatter check. (default: true)
    --linter-enabled                         Allow to enable or disable the linter check. (default: true)
    --indent-style <tabs|space>              Change the indention character (default: tabs)
    --indent-size <number>                   If the indentation style is set to spaces, determine how many spaces should be used for indentation (default: 2)
    --line-width <number>                    Change how many characters the formatter is allowed to print in a single line (default: 80)
    --quote-style <single|double>            Changes the quotation character for strings (default: ")
    --quote-properties <as-needed|preserve>  Changes when properties in object should be quoted (default: as-needed)
    --trailing-comma <all|es5|none>          Changes trailing commas in multi-line comma-separated syntactic structures (default: all)
```