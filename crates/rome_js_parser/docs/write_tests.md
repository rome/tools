
# Write tests for a parser

If you want to create a new test for an existing parser, you will have to inline
the code that you want to test in a comment that is created in a specific way.

Let's say that you created a new parsing feature and you need new tests from scratch,
just go to the source code where you parse this new feature if JavaScript, and add the following comment:

```rust
// test feature_name
// let a = { new_feature : "" }
// let b = { new_feature : "" }
fn parse_new_feature(p: &mut Parser) -> ParsedSyntax {}
```

The first line, `// test feature_name` the important one. This will tell to the
testing infrastructure to create a **positive test** (without parsing errors), called
`feature_name.js` inside the `test_data/inline/ok` folder.

The content of this file will be:

```js
let a = { new_feature : "" }
let b = { new_feature : "" }
```

Basically, everything after the key comment will be the content of the new file.

Now you need to run `cargo codegen test` and the task will actually generate this file for you.

In case you want to create a **negative test** (*with* parsing errors), you will
create a new comment like this:

```diff
// test feature_name
// let a = { new_feature : "" }
// let b = { new_feature : "" }

+ // test_err feature_name
+ // let a = {  : "" }
+ // let b = { new_feature :  }
fn parse_new_feature(p: &mut Parser) -> ParsedSyntax {}
```

Mind the different comment **`test_err`**, which marks the error for the test suite
as a test that has to fail.

Run the command `cargo codegen test` and you will see a new file called
`feature_name.js` inside the `test_data/inline/err` folder.

The content of this file will be:

```js
let a = {  : "" }
let b = { new_feature :  }
```

Now run the command:
Unix/macOS

```bash
env UPDATE_EXPECT=1 cargo test
```

Windows

```powershell
set UPDATE_EXPECT=1 & cargo test
```
The command will tell the test suite to generate and update the `.rast` files.

If tests that are inside the `ok/` folder fail or if tests that are inside the `err/`
folder don't emit, the whole test suite will fail.
