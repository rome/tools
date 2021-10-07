# Tests macros

A set of utilities that automatically generate unit tests from files.

## Usage

First argument: glob that will passed to https://github.com/gilnaa/globwalk. Crate's cargo.toml will be the base directory. To pattern format see here: https://git-scm.com/docs/gitignore#_pattern_format
Second argument: method that will be called.

> The test name is the `snake_case` version of the file name.

```rust
tests_macros::gen_tests!{"tests/*.{js,json}", run_test}

fn run_test<S: AsRef<str> + std::fmt::Debug>(a: S, b: S) {
    println!("{:?} {:?}", a, b);
}
```

This will generate the following for each file:

```rust
#[test]
pub fn some_test()
{
    let test_file = "<SOMEDIR>/tests/sometest.txt";
    let test_expected_file = "<SOMEDIR>/tests/sometest.expected.txt";
    run_test(test_file, test_expected_file);
}
```
