# coverage


## `cargo coverage`
Runs the parser against the [Test262](https://github.com/tc39/test262) and [TypesCript tests](https://github.com/microsoft/TypeScript/tree/main/tests)
test suites to validate its conformance.

You can run all test suites with

```bash
cargo coverage
```

...or a specific test suite by specifying the language:

```bash
cargo coverage --language=js
```

You can filter the tests by passing an additional `--filter` so that only the tests matching the passed name are executed:

```bash
cargo coverage --filter=source
```

The `--detailed` parameter enables a more detailed summary at the end of the test run that includes diagnostic.
You can use `--detailed=debug` if you're debugging a test (prints the AST and diagnostics, even for passing tests). `--detailed` can also be useful if you want to pipe the results to a text file.


```bash
cargo coverage --detailed
```


## `cargo coverage compare`

Useful to compare the coverage, for example between your feature branch and `main`.

```bash
# (commit your code on pr branch, run)
git checkout main
cargo coverage --json > base_results.json
git checkout <your branch>
cargo coverage --json > new_results.json
cargo coverage compare ./base_results.json ./new_results.json --markdown
```
