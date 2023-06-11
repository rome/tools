# rome-fuzz

Fuzzers and associated utilities for automatic testing of Rome.

## Usage

To use the fuzzers provided in this directory, start by invoking:

```bash
./fuzz/init-fuzzers.sh
```

This will install [`cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz) and optionally download a
[dataset](https://www.sri.inf.ethz.ch/js150) which improves the efficacy of the testing.
**This step is necessary for initialising the corpus directory, as all fuzzers share a common
corpus.**
The dataset may take several hours to download and clean, so if you're just looking to try out the
fuzzers, skip the dataset download, though be warned that some features simply cannot be tested
without it (very unlikely for the fuzzer to generate valid python code from "thin air").

Once you have initialised the fuzzers, you can then execute any fuzzer with:

```bash
cargo fuzz run -s none name_of_fuzzer -- -timeout=1
```

**Users using Apple M1 devices must use a nightly compiler and omit the `-s none` portion of this
command, as this architecture does not support fuzzing without a sanitizer.**
You can view the names of the available fuzzers with `cargo fuzz list`.
For specific details about how each fuzzer works, please read this document in its entirety.

**IMPORTANT: You should run `./reinit-fuzzer.sh` after adding more file-based testcases.** This will
allow the testing of new features that you've added unit tests for.

### Debugging a crash

Once you've found a crash, you'll need to debug it.
The easiest first step in this process is to minimise the input such that the crash is still
triggered with a smaller input.
`cargo-fuzz` supports this out of the box with:

```bash
cargo fuzz tmin -s none name_of_fuzzer artifacts/name_of_fuzzer/crash-...
```

From here, you will need to analyse the input and potentially the behaviour of the program.
The debugging process from here is unfortunately less well-defined, so you will need to apply some
expertise here.
Happy hunting!

## A brief introduction to fuzzers

Fuzzing, or fuzz testing, is the process of providing generated data to a program under test.
The most common variety of fuzzers are mutational fuzzers; given a set of existing inputs (a
"corpus"), it will attempt to slightly change (or "mutate") these inputs into new inputs that cover
parts of the code that haven't yet been observed.
Using this strategy, we can quite efficiently generate testcases which cover significant portions of
the program, both with expected and unexpected data.
[This is really quite effective for finding bugs.](https://github.com/rust-fuzz/trophy-case)

The fuzzers here use [`cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz), a utility which allows
Rust to integrate with [libFuzzer](https://llvm.org/docs/LibFuzzer.html), the fuzzer library built
into LLVM.
Each source file present in [`fuzz_targets`](fuzz_targets) is a harness, which is, in effect, a unit
test which can handle different inputs.
When an input is provided to a harness, the harness processes this data and libFuzzer observes the
code coverage and any special values used in comparisons over the course of the run.
Special values are preserved for future mutations and inputs which cover new regions of code are
added to the corpus.

## Each fuzzer harness in detail

TODO
