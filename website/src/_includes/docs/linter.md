## Linter

You can use the linter via our [VS Code extension] or by downloading our CLI directly from our [release page].

> WARNING: The CLI and VS Code extension are packaged with separate binaries, which means that if you don't
> use our default options, you will have to **pass them to both the extension AND the CLI**.
>
> This is a temporary choice to allow people to play with our linter. This will change in the near future.


### Use the linter via VSCode extension

The feature is opt-in, and you'd need to enable the following options:
- `analysis.enableDiagnostics` 
- `analysis.enableCodeActions` 

### Use the linter via CLI

You can start by running the CLI with the `--help` flag:

```shell
rome check --help
```

Which will show you the options available at the moment:

```shell
Rome Check: Run the linter on a set of files

USAGE:
    rome check <INPUTS...>

    INPUTS can be one or more filesystem path, each pointing to a single file or an entire directory to be searched recursively for supported files

OPTIONS:
    --apply                       Apply safe fixes
    --apply-suggested             Apply safe and suggested fixes
    --max-diagnostics             Cap the amount of diagnostics displayed - default 20

```

### Rules

At the moment only a few rules are implemented as the linting / analysis infrastructure is being built.

**See the full [list of rules](/docs/lint/rules).**

All rules are enabled by default, and cannot be disabled. [Suppression](#lint-suppression) can be used to hide specific lint errors.


[VS Code extension]: https://marketplace.visualstudio.com/items?itemName=rome.rome
[release page]: https://github.com/rome/tools/releases


### Lint suppression

There are times when a developer wants to ignore a lint rule for a specific line of the code.

You can achieve this by adding a suppression comment above the line that is triggering the lint diagnostic.

Suppression comments have the following format:

```js
// rome-ignore lint: <explanation>
// rome-ignore lint(js/noDebugger): <explanation>
```

Where
- `rome-ignore` is the start of a suppression comment;
- `lint:` suppresses the linter;
- `(js/noDebugger)`: **optional**, group and name of the rule you want to suppress;
- `<explanation>` explanation why the rule is disabled

Here's an example:

```ts
// rome-ignore lint: reason
declare const Foo: number;
// rome-ignore lint(js/noUnusedVariables): reason
declare const Bar: number;
```


### Code fixes

Lint rules may provide automatic code fixes. Rome distinguishes between two types of fixes:

* safe fixes
* suggested fixes

Safe fixes are guaranteed to not change the semantics of your code,
and can be applied without explicit review.

Suggested fixes may change the semantics of your program, and it's, 
therefore, advised to manually review the changes. 