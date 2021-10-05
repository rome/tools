Automatically generate unit tests from files. 

# Usage

First argument: glob that will passed to https://github.com/gilnaa/globwalk. Crate's cargo.toml will be the base directory.  
Second argument: method that will be called.

```
tests_macros::gen_tests!{"tests/*", run_test}

fn run_test<S: AsRef<str> + std::fmt::Debug>(a: S, b: S) {
    println!("{:?} {:?}", a, b);
}
```

this will generate the following for each file:
Test name is the "snake case" version of the file name.

```
#[test]
pub fn sometest()
{
    let test_file = "<SOMEDIR>/tests/sometest.txt";
    let test_expected_file = "<SOMEDIR>/tests/sometest.expected.txt";
    run_test(test_file, test_expected_file);
}
```