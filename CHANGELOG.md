# Rome changelog

## 0.9.0

### CLI

- You can now format content from standard input when using the command `rome format`:
```shell
echo "function f() { return {} }" | rome format --stdin-file-path example.js
```
the argument  `--stdin-file-path` is mandatory when from standard in. The path should represent a 
file name with their extension.
- Added `--apply-suggested` argument to the `rome check` command, to apply suggested and safe fixes.

### Linter

- Changed the default severity for recommended rules to "error". You can [change the severity in the rome.json](https://rome.tools/#configure-a-rule).
- Added [`js/noExtraBooleanCast`](https://rome.tools/docs/lint/rules/noExtraBooleanCast/) lint rule.
- Added [`js/noDupeArgs`](https://rome.tools/docs/lint/rules/noDupeArgs/) lint rule.
- Added [`js/noShadowRestrictedNames`](https://rome.tools/docs/lint/rules/noShadowRestrictedNames/) lint rule.
- Added `js/inlineVariable` code action.
- Applied various stability fixes to the rule [`js/noUnusedVariables`](https://rome.tools/docs/lint/rules/noUnusedVariables/). [#3124](https://github.com/rome/tools/pull/3124) [#3060](https://github.com/rome/tools/pull/3060) [#3004](https://github.com/rome/tools/pull/3004)
- Fixed how the suggestion is applied [`js/noNegationElse`](https://rome.tools/docs/lint/rules/noNegationElse/). [#2999](https://github.com/rome/tools/issues/2999)
- Fixed false positive in the rule [`js/noShoutyConstants`](https://rome.tools/docs/lint/rules/noShoutyConstants/). [#3077](https://github.com/rome/tools/issues/3077)

### Formatter

Greatly increased the compatibility with Prettier, here's an highlight of what we improved

##### `throw` and `return` arguments

##### Templates

##### Arrow functions

##### Conditional expressions

##### Parenthesis

##### Arrays

##### Loops


### Configuration

- **BREAKING CHANGE**: removed the second `"rules"` field from a field group.
```diff
{
  "linter": {
    "enabled": true,
    "rules": {
      "js": {
+        "noDebugger": "off"
-        "rules": {
-          "noDebugger": "off"
-        },
      }
    }
  }
}
```

## 0.8.0

### CLI

- Added `--max-diagnostics` argument to the command `rome check`.
- The maximum number of diagnostics printed is now 20, use `--max-diagnostics` to change the default. 
- Added a new command `rome init`.

### Configuration

- You can create a configuration file called `rome.json` to customize Rome's default options. 
This will work from both CLI and LSP.

### Linter

The linter is now marked as "alpha" and it can be used to lint code from the CLI and
from the LSP.

### Formatter

- You can now use the configuration file `rome.json` to change Rome's defaults:

  Example:
  ```json
  {
    "root": true,
    "formatter": {
      "indentStyle": "space"
    }
  }
  ```
- Fixed some edge cases where the comment suppressions were not working as expected.

### VSCode

- **BREAKING CHANGE**: Removed the majority of settings that were available in the extension, use the 
configuration file `rome.json` to change the Rome's defaults.
- The extension now allows to rename variables;

## 0.7.0

### CLI

- Added `--no-colors` argument.

### Formatter

- JSX and TSX are now formatted by default! Make sure to enable Rome as default formatter in the VSCode extension.
- Improved the consistency of formatting of various statements:
  - call arguments;
  - object property members;
  - variable declarations;
  - object patterns;
  - class property members;
- Fixed a bunch of issues in the TypeScript formatting.

### Linter

- Added the new `--apply` argument to the `rome check` command; 
- New rules added to the linter, check the [website](https://rome.tools/docs/lint/rules/);

## 0.6.1

Fixes a regression introduced in the `rome format` command ([#2670](https://github.com/rome/tools/issues/2670))

## 0.6.0

### Formatter

- BREAKING CHANGES: the command `rome format --ci` has been removed, use `rome ci` instead.

#### Improved the compatibility with Prettier (check [#2403](https://github.com/rome/tools/issues/2403) for more details)
  
- TypeScript's formatting is better in line with what Prettier does.
- Better formatting of string literals.
Removing unnecessary quotes in string literals and quotes from member names. 
Correctly choose the correct quote based on quantity of quotes inside a literal:
  ```js
  // original code
  let a = {
    "something": 3
  }
  let b = "cool isn\'t it";
  let c = "\"content\" ' ";
  
  // formatted code
  let a = {
    something: 3
  }
  let b = "cool isn't it";   
  let c = '"content" \' ';
  ```
- Better formatting of various statements
- Improved the performance of the formatter an average of 20%-30%! Check the relevant
PRs [1](https://github.com/rome/tools/pull/2456), [2](https://github.com/rome/tools/pull/2638), [3](https://github.com/rome/tools/pull/2612), [4](https://github.com/rome/tools/pull/2462), [5](https://github.com/rome/tools/pull/2634) if you're interested in what the team did.
  
To reach better compatibility with Prettier, the team had to revise the foundation of our printer,
which caused some regressions around how comments are printed. These are known issues that we
plan to close by next release.

### Linter

We've built the foundation of our linter. At the moment is only opt-in, and it contains
only a bunch of rules. **Safe fixes are not enabled yet via CLI**.

Refer to the [website](https://rome.tools/#linter) to learn how to start using it.

## 0.5.0

- BREAKING CHANGES: the `format` command doesn't write on disk by default. Now the command prints on terminal.

    **Migration**: add the `--write` argument when calling `rome format`
    
    ```shell
    rome format --write
    ```

- Added a new option called `--quote-style` to the formatter. This option is also available on VSCode.

## 0.4.0

Rome has been [rewritten in Rust](https://rome.tools/blog/2021/09/21/rome-will-be-rewritten-in-rust)!

The great majority of the previous functionality won't work anymore, as we rewrote the whole software
from scratch.

Rome, for now, exposes a new formatter that has been revisited and, is way faster compared to its former version!

To install it, use the `next` tag on `npm`:

```shell
npm i rome@next
```

Or follow our [getting started](https://rome.tools/#getting-started) section for more details.
