# coverage

## `cargo coverage js` / `cargo coverage ts`
These commands check the parser coverage against [Test262](https://github.com/tc39/test262) or [TypesCript tests](https://github.com/microsoft/TypeScript/tree/main/tests).

If you would like to investigate some specific files,
run the command with a pathname filter:

```bash
cargo coverage js invalid-regexp --show-rast --show-diagnostics
```

`--show-rast` and `--show-diagnostics` outputs intermediate results.

## `cargo coverage compare`
When we want to compare parser changes between our feature branch and the main branch,
we can run the compare command to get the coverage difference:

```bash
# (commit your code on pr branch, run)
git checkout main
cargo coverage js --json > base_results.json
git checkout -
cargo coverage js --json > new_results.json
cargo coverage compare ./base_results.json ./new_results.json --markdown
```
