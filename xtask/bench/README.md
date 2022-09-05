# Benchmark

This crate contains benchmark suites for the project.

Criterion is used to generate benchmark results.

## Parser Benchmark

To get a benchmark comparison, you need to run the benchmark for `main` branch and your PR:

```bash
# (commit your code on pr branch, run)
git checkout main
cargo bench_parser --save-baseline main
git checkout -
cargo bench_parser --save-baseline pr
critcmp main pr # (cargo install critcmp)
```

This will give us

```
group                                 main                                    pr
-----                                 ----                                    --
parser/compiler.js                    1.05    98.1±26.02ms    10.7 MB/sec     1.00     93.4±7.47ms    11.2 MB/sec
parser/d3.min.js                      1.09    57.7±10.50ms     4.5 MB/sec     1.00     52.9±1.59ms     5.0 MB/sec
parser/dojo.js                        1.13      4.8±0.96ms    14.4 MB/sec     1.00      4.2±0.30ms    16.3 MB/sec
parser/jquery.min.js                  1.16     20.5±4.63ms     4.0 MB/sec     1.00     17.6±0.55ms     4.7 MB/sec
parser/pixi.min.js                    1.00    72.3±15.58ms     6.1 MB/sec     1.02     73.7±9.94ms     6.0 MB/sec
parser/react-dom.production.min.js    1.00     22.3±1.95ms     5.2 MB/sec     1.04     23.2±4.56ms     5.0 MB/sec
parser/react.production.min.js        1.05  1096.7±295.86µs     5.6 MB/sec    1.00  1049.1±237.49µs     5.9 MB/sec
parser/tex-chtml-full.js              1.08   154.9±21.43ms     5.9 MB/sec     1.00   143.9±22.96ms     6.3 MB/sec
parser/three.min.js                   1.00    81.6±11.86ms     7.2 MB/sec     1.10    90.2±19.85ms     6.5 MB/sec
parser/vue.global.prod.js             1.09     28.7±6.39ms     4.2 MB/sec     1.00     26.3±0.88ms     4.6 MB/sec
```

The 1.xx column is the percentage difference, larger means worse.
For example jquery is 16% slower on main. And the pr branch performs better overall.

## Formatter benchmark

To get a benchmark comparison, you need to run the benchmark for `main` branch and your PR:

```bash
# (commit your code on pr branch, run)
git checkout main
cargo bench_formatter --save-baseline main
git checkout -
cargo bench_formatter --save-baseline pr
critcmp main pr # (cargo install critcmp)
```

## Heap Profiling using `dhat`

```bash
cargo run -p xtask_bench --features dhat-heap --release-with-debug
```
