## Linter

Rome's linter statically analyzes your code to catch common errors and help write more idiomatic code.

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

Rules in Rome are divided into categories to help you understand their purpose.

**See the full [list of rules](/docs/lint/rules).**

All rules are enabled by default, and cannot be disabled. [Suppression](#lint-suppression) can be used to hide specific lint errors.


[VS Code extension]: https://marketplace.visualstudio.com/items?itemName=rome.rome
[release page]: https://github.com/rome/tools/releases


### Code fixes

Lint rules may provide automatic code fixes. Rome distinguishes between two types of fixes:

* safe fixes
* suggested fixes

Safe fixes are guaranteed to not change the semantics of your code,
and can be applied without explicit review.

Suggested fixes may change the semantics of your program, and it's,
therefore, advised to manually review the changes.

### Ignoring Code

There are times when a developer wants to ignore a lint rule for a specific line of the code.

You can achieve this by adding a suppression comment above the line that is triggering the lint diagnostic.

Suppression comments have the following format:

```js
// rome-ignore lint: <explanation>
// rome-ignore lint(correctness/noDebugger): <explanation>
```

Where
- `rome-ignore` is the start of a suppression comment;
- `lint:` suppresses the linter;
- `(correctness/noDebugger)`: **optional**, group and name of the rule you want to suppress;
- `<explanation>` explanation why the rule is disabled

Here's an example:

```ts
// rome-ignore lint: reason
debugger;
// rome-ignore lint(correctness/noDebugger): reason
debugger;
```


