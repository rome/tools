---
title: Formatter
---

# Formatter

Rome's an opinionated formatter that has the goal to stop all ongoing debates over styles. That's why Rome only supports
few options to avoid that debates over styles turn into debates over Rome options.

## Options

The language agnostic options supported by Rome are:

- indent style (default: `tabs`): Use spaces or tabs for indention
- tab width (default: `2`): The number of spaces per indention level
- line width (default: `80`): The column width at which Rome wraps code

## Use the formatter with the CLI

You can start by running the CLI with the `--help` flag:

```shell
rome format --help
```

Which will show you the options available at the moment:

```shell
Rome Formatter

USAGE:
    rome format [OPTIONS] <INPUTS...>

    INPUTS can be one or more filesystem path, each pointing to a single file or an entire directory to be searched recursively for supported files

OPTIONS:
    --write                                  Edit the files in place (beware!) instead of printing the diff to the console
    --skip-errors                            Skip over files containing syntax errors instead of emitting an error diagnostic.
    --indent-style <tabs|space>              Change the indention character (default: tabs)
    --indent-size <number>                   If the indentation style is set to spaces, determine how many spaces should be used for indentation (default: 2)
    --line-width <number>                    Change how many characters the formatter is allowed to print in a single line (default: 80)
    --quote-style <single|double>            Changes the quotation character for strings (default: ")
    --quote-properties <as-needed|preserve>  Changes when properties in object should be quoted (default: as-needed)
    --trailing-comma <all|es5|none>          Changes trailing commas in multi-line comma-separated syntactic structures (default: all)
    --stdin-file-path <string>               A file name with its extension to pass when reading from standard in, e.g. echo 'let a;' | rome format --stdin-file-path file.js
```

## Ignoring Code

There are times when a developer wants to keep a specific formatting.

You can achieve this by adding a suppression comment right before the syntax node (expressions, statements, etc.).

Suppression comments have the following format:

```js
// rome-ignore format: <explanation>
```

Where

- `rome-ignore` is the start of a suppression comment;
- `format:` suppresses the formatting;
- `<explanation>` is an explanation why the formatting is disabled;

Here's an example of how a code will look like before and after the formatter does its job:

Before running the formatter

```js
const expr =
  // rome-ignore format: the array should not be formatted
  [
    (2 * n) / (r - l),
    0,
    (r + l) / (r - l),
    0,
    0,
    (2 * n) / (t - b),
    (t + b) / (t - b),
    0,
    0,
    0,
    -(f + n) / (f - n),
    -(2 * f * n) / (f - n),
    0,
    0,
    -1,
    0,
  ];

const expr = [
  (2 * n) / (r - l),
  0,
  (r + l) / (r - l),
  0,
  0,
  (2 * n) / (t - b),
  (t + b) / (t - b),
  0,
  0,
  0,
  -(f + n) / (f - n),
  -(2 * f * n) / (f - n),
  0,
  0,
  -1,
  0,
];
```

After running the formatter

```js
const expr =
  // rome-ignore format: the array should not be formatted
  [
    (2 * n) / (r - l),
    0,
    (r + l) / (r - l),
    0,
    0,
    (2 * n) / (t - b),
    (t + b) / (t - b),
    0,
    0,
    0,
    -(f + n) / (f - n),
    -(2 * f * n) / (f - n),
    0,
    0,
    -1,
    0,
  ];

const expr = [
  (2 * n) / (r - l),
  0,
  (r + l) / (r - l),
  0,
  0,
  (2 * n) / (t - b),
  (t + b) / (t - b),
  0,
  0,
  0,
  -(f + n) / (f - n),
  -(2 * f * n) / (f - n),
  0,
  0,
  -1,
  0,
];
```

As you can see the first array, which has a suppression comment, is left untouched!

## Migration from other formatters

Rome doesn't support a lot of options like other web formatters, which means that particular styles
won't be available to all developers.

To migrate from suppression comments of the old formatter, it's recommended to run a global search and replace against the code
base and replace the formatting comment with:

```
// rome-ignore format: migration from <name_of_former_formatter>
```

Then, you are free to change the reason of the suppression that you want.

Run the formatter and make sure that **the code that was ignored is still the same**.

[vs code extension]: https://marketplace.visualstudio.com/items?itemName=rome.rome
[release page]: https://github.com/rome/tools/releases
[playground]: https://play.rome.tools
[command palette]: https://code.visualstudio.com/getstarted/userinterface#_command-palette
