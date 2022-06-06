# Rome changelog

## 0.6.0

- BREAKING CHANGES: the command `rome format --ci` has been removed, use `rome ci` instead.
- Improved the compatibility with Prettier (check #2403 for more details):
  - TypeScript's formatting is now in line with what Prettier does.
  - Better formatting of string literals. The formatter now does a lot of cleaning:
  ```js
  // original code
  let a = {
    "something": 3
  }
  let b = "cool ins\'t it";
  
  // formatted code
  let a = {
    something: 3
  }
  let b = "cool ins't it";
  ```
  - Better formatting of various statements
  - Improved the performance of the formatter an average of 20%! Check the [relevant
PR](https://github.com/rome/tools/pull/2634) if you're interested in what the team did.

To reach better compatibility with Prettier, the team had to revise the foundation of our printer,
which caused some regressions around how comments are printed. These are known issues that we
plan to close by next release.

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