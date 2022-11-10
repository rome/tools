# Benchmarks
## Getting Started
1. Install hyperfine: `cargo install hyperfine`
2. Install node modules: `npm i`
3. Run the benchmarks: `node run.js`

## Results
Setup: MacBook Pro (13-inch, M1, 2020)

### Formatting
* Rome's ~25 times faster than Prettier
* Rome's ~20 times faster than parallel-prettier
* Rome's ~20 times faster than `xargs -P`[^1]
* Rome's 1.5-2 times faster than `dprint`
* Rome single-threaded is ~7 times faster than Prettier.


[^1]: Run `time find lib/ examples declarations benchmark -name '*.js' -print0 | xargs -P8 -0 -n 200 npx prettier --write --loglevel=error` in the `target/webpack` directory. I manually tinkered with the `-n` parameter to get the fastest run.

### Linting
* Rome's ~15x times faster than ESLint
* Rome single-threaded is ~4 times faster than ESLint.

The speed-ups for the multithreaded benchmarks can vary significantly depending on the setup. For example, Rome is 100 times faster than Prettier on an M1 Max with 10 cores.

## Analysis
### Formatter
* Rome's formatter is fast :).
* It should be possible to speed up Prettier. Rome's architecture isn't that different, and native has its advantages, but Prettier should be able to compete in single-threaded mode.

### Linting
* Rome's linter is fast but there is room for improvements
* Rome's linter spends significant time building the semantic model, the control flow graph, and matching queries. I'm convinced there's room for improvement ([3565](https://github.com/rome/tools/pull/3565), [3569](https://github.com/rome/tools/pull/3569)).
* Computing the diff for code fixes is expensive. Rome can spend up to 3s computing diffs (not measured by these benchmarks, see explanations below)

## Notes

We've been careful to create fair benchmarks. This section explains some of the decisions behind the benchmark setup and how these decisions affect the results. Please [let us know](https://github.com/rome/tools/issues) if you have ideas on how to make the benchmarks fairer or if there's a mistake with our setup.

### Formatting
* Compares the wall time of Rome, Prettier, and dprint to format all files in a project where all files are correctly formatted.
* dprint and Prettier support incremental formatting to only format changed files, whereas Rome does not. This benchmark does not measure incremental formatting as it measures cold formatting time. You may see significant speedups on subsequent formatting runs when enabling incremental formatting.
* The benchmark limits Prettier to only format JavaScript and TypeScript files because Rome doesn't support other file types.
* Rome only prints a summary with the number of formatted files. The prettier benchmark uses `--loglevel=error` for a fairer benchmark so that Prettier doesn't print every filename.

### Linting
* Compares the wall time of Rome and ESLint to check all files in a project.
* Only enables rules that both Rome and ESLint support.
* Rome prints rich diffs for each lint diagnostic, whereas ESLint only shows the name and description of the lint rule. That's why the benchmark uses `--max-diagnostics=0` when running Rome because Rome then only counts diagnostics without generating the diffs. Overall, this results in a more accurate comparison but has the downside that Rome only prints the diagnostic count, whereas ESLint prints a line for each lint error.

