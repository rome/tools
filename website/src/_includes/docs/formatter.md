## Formatter

You can use the formatter via our [VS Code extension] or by downloading our CLI directly from our [release page].

> WARNING: The CLI and VS Code extension are packaged with separate binaries, which means that if you don't
> use our default options, you will have to **pass them to both the extension AND the CLI**.
>
> This is a temporary choice to allow people to play with our formatter. This will change in the near future.


### Formatter options

Our formatter is really strict and has support for only a few options:

- indent style, you can choose between tabs or spaces; **Rome's default is tabs**
- quantity of spaces, applied only if you choose spaces as indent style;
- line width, which is the number of characters that fit in a single line; **Rome's default is `80`**

### Use the formatter via VSCode extension

The extension allows you to change the default [formatter options](#formatter-options).

For easy access to the available options, navigate to the settings menu of the VSCode extension and type: `@ext:rome.rome`.

Plus, you can try an additional feature that allows you to format code with syntax errors.

This is an **opt-in feature** that allows developers to experiment with a formatter that can work with an error resilient parser.

> WARNING: all options are marked as **BETA** because this might change, once we will add support of a configuration file

If you want to set Rome as your default formatter, you can do so by opening the [command palette]
and select `Format Document With ...` , then `Configure Default Formatter` and finally select `Rome`. The option will
appear only for documents that Rome supports.


### Use the formatter with the CLI

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
    --stdin-file-path <string>                A file name with its extension to pass when reading from standard in, e.g. echo 'let a;' | rome format --stdin-file-path file.js
```

### Suppression

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
const   expr   =
// rome-ignore format: the array should not be formatted
[
    (2*n)/(r-l), 0,            (r+l)/(r-l),  0,
    0,           (2*n)/(t-b),  (t+b)/(t-b),  0,
    0,           0,           -(f+n)/(f-n), -(2*f*n)/(f-n),
    0,           0,           -1,            0,
];


const   expr   =   [
    (2*n)/(r-l), 0,            (r+l)/(r-l),  0,
    0,           (2*n)/(t-b),  (t+b)/(t-b),  0,
    0,           0,           -(f+n)/(f-n), -(2*f*n)/(f-n),
    0,           0,           -1,            0,
];
```

After running the formatter

```js
const expr =
// rome-ignore format: the array should not be formatted
[
    (2*n)/(r-l), 0,            (r+l)/(r-l),  0,
    0,           (2*n)/(t-b),  (t+b)/(t-b),  0,
    0,           0,           -(f+n)/(f-n), -(2*f*n)/(f-n),
    0,           0,           -1,            0,
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

#### Migration from other formatters

Rome doesn't support a lot of options like other web formatters, which means that particular styles
won't be available to all developers.

To migrate from suppression comments of the old formatter, it's recommended to run a global search and replace against the code
base and replace the formatting comment with:

```
// rome-ignore format: migration from <name_of_former_formatter>
```

Then, you are free to change the reason of the suppression that you want.

Run the formatter and make sure that **the code that was ignored is still the same**.


[VS Code extension]: https://marketplace.visualstudio.com/items?itemName=rome.rome
[release page]: https://github.com/rome/tools/releases
[playground]: https://play.rome.tools
[command palette]: https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette
