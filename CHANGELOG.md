# Rome changelog

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
