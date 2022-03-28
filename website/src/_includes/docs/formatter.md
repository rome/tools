## Formatter

You can use the Rome formatter via our [VS Code extension](https://marketplace.visualstudio.com/items?itemName=rome.rome)
or by downloading our CLI directly from our  [release page](https://github.com/rome/tools/releases).

> WARNING: both the CLI and the VS Code extension are packaged with separated binaries, which means that if you don't 
> use our default options, you will have to make sure to **pass them to both the extension AND the CLI**.
> 
> This is a temporary choice to allow people to play with our formatter. This will change in the near future.


### Formatter options

Our formatter is really strict and has support for few options:

- indent style, you can choose between tabs or spaces; **rome's default is tabs**
- quantity of spaces, applied only if you choose spaces as indent style; 
- line width, which is the number of characters that fit in a single line; **rome's default is `80`**

### Use the formatter with the VSCode extension

The extension allows to change the default [formatter options](#formatter-options). 

To easy access to the available options, to the settings menu of the VSCode extension and type: `@ext:rome.rome`.

Plus, you can try an additional feature that allows you to format partial broken code (code with syntax errors).
This is an **opt-in feature** that allows the developers to experiment how a formatter can work with an error resilient parser.

> WARNING: all options are marked as **BETA** because this might change, once we will add support of a configuration file

If you want to set Rome formatter as your default formatter, you can do so by opening the [command palette]
and select `Format Document With ...` , then `Configure Default Formatter` and finally select `Rome`. The option will
appear only for documents that Rome supports.


### Use the formatter with the CLI

The only command that is supported is `format`.

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
    --ci                        Enable CI mode, lock files and exit with an error if the formatter would modify them
    --skip-errors               Skip over files containing syntax errors instead of returning an error
    --indent-style <tabs|space> Determine whether the formatter should use tabs or spaces for indentation (default: tabs)
    --indent-size <number>      If the indentation style is set to spaces, determine how many spaces should be used for indentation (default: 2)

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

### Differences with Prettier/dprint

Rome formatter uses a CST to implement its algorithms - as supposed to Prettier or dprint, which use an 
AST -, which means that it has to deal with a different set of problems e.g. comments and how to place them.

As you might know, comments can appear almost everywhere inside a program, which can make the implementation
of a formatter more difficult.

In a CST, comments are attached to tokens, so it's possible to extract this information when inspecting
a single node. 

Considering these assumptions, the Rome team had to create some heuristics and concepts in order to 
**consistently format comments inside a program**.

#### Comments with hard groups

A **hard group** is a concept that the Rome team created when formatting certain parts of the code. The 
idea of a hard group is that tokens that belong to this group can't be broken on multiple lines and, they can't
contain comments. 

We apply this concept, for example, to JavaScript functions and JavaScript classes. A function has a "head" and a "body":
- the head is where we define the name of the function and its signature (its parameters, return type, etc.);
- the body is where we define the implementation of the function, usually - but not only - inside a block `{}`;

Our formatter marks a function head as a hard group, while the body is a normal group. This means that all 
the comments inside the head are "pushed out" and moved outside it, making the formatting **always consistent**.

Here's an example against Prettier/dprint, we place comments inside the head of a function:

```js
function // something
 a(b, c)  {
  let a = "f";
}

function a(b, c) // something 
{
    let a = "f";
}
```

This how Rome and Prettier format this code:
```js
// Rome
function a(b, c) {
	// something
	let a = "f";
}

function a(b, c) {
    // something
    let a = "f";
}


// Prettier/dprint
function // something
a(b, c) {
    let a = "f";
}
function a(b, c) {
    // something
    let a = "f";
}
```

Please check our [playground] and its result

#### Migrate from other formatters

Rome doesn't support a lot of options like other web formatters, which means that particular styles 
won't be available to all developers.

To migrate from suppression comments of the old formatter, it's recommended to run a global search and replace against the code
base and replace the formatting comment with:

```block
// rome-ignore format: migration from <name_of_former_formatter>
```

Then, you are free to change the reason of the suppression that you want.

Run Rome formatter and make sure that **the code that was ignored is still the same**.



[playground]: https://play.rome.tools/?lineWidth=80&indentStyle=tab&indentWidth=2&typescript=true&jsx=false#ZnVuY3Rpb24gLy8gc29tZXRoaW5nCiBhKGIsIGMpICB7CiAgbGV0IGEgPSAiZiI7Cn0KCmZ1bmN0aW9uIGEoYiwgYykgLy8gc29tZXRoaW5nIAp7CiAgICBsZXQgYSA9ICJmIjsKfQ==


[command palette]: https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette