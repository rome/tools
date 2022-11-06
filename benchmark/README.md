# Benchmarks
## Getting Started
1. Install hyperfine: `cargo install hyperfine`
2. Install node modules: `npm i`
3. Run the benchmarks: `node run.js`

## Results
Setup: MacBook Pro (13-inch, M1, 2020)

### Formatting
* Rome's ~25 times faster than Prettier
* Rome's ~7 times faster when restricting it to a single core

### Linting
* Rome's 2-3 times faster than ESLint
* Rome is ~20-40% slower than ESLint when restricting it to a single core.
The speed-ups for the multithreaded benchmarks can vary significantly depending on the setup. For example, Rome is 100 times faster than Prettier on an M1 Max with 10 cores.

## Analysis
### Formatter
* Rome's formatter is fast :).
* It should be possible to speed up Prettier. Rome's architecture isn't that different, and native has its advantages, but Prettier should be able to compete in single-threaded mode.

### Linting
* Rome's linter spends significant time building the semantic model, the control flow graph, and matching queries. I'm convinced there's room for improvement ([3565](https://github.com/rome/tools/pull/3565), [3569](https://github.com/rome/tools/pull/3569)).
* Computing the diff for code fixes is expensive. Rome can spend up to 3s computing diffs (not measured by these benchmarks, see explanations below)
* Hypothesis: Rome doesn't use async IO to read files. That's why the single-threaded Rome issues the file read commands one by one. ESLint may be faster in single-threaded linting because it can issue all reads with async IO (so that the OS loads the files in the background while other files are linted). I have yet to verify if ESLint indeed does use async IO.

## Notes

We've been careful to create fair benchmarks. This section explains some of the decisions behind the benchmark setup and how these decisions affect the results. Please [let us know](https://github.com/rome/tools/issues) if you have ideas on how to make the benchmarks fairer or if there's a mistake with our setup.

### Formatting
* Compares the wall time of Rome and Prettier to format all files in a project where all files are correctly formatted.
* The benchmark limits Prettier to only format JavaScript and TypeScript files because Rome doesn't support other file types.
* Rome only prints a summary with the number of formatted files. The prettier benchmark uses `--loglevel=error` for a fairer benchmark so that Prettier doesn't print every filename.

### Linting
* Compares the wall time of Rome and ESLint to check all files in a project.
* Only enables rules that both Rome and ESLint support.
* Rome prints rich diffs for each lint diagnostic, whereas ESLint only shows the name and description of the lint rule. That's why the benchmark uses `--max-diagnostics=0` when running Rome because Rome then only counts diagnostics without generating the diffs. Overall, this results in a more accurate comparison but has the downside that Rome only prints the diagnostic count, whereas ESLint prints a line for each lint error.

