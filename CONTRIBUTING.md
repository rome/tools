# Contributing

We can use help in a bunch of areas and any help is appreciated. Our [GitHub issues](https://github.com/rome/tools/issues) serve as a place for any discussion, whether it's bug reports, questions, project direction etc. As the project grows this policy may change.

Our [Discord server](https://discord.gg/rome) is open for help and more adhoc discussion. All activity on the Discord is still moderated and will be strictly enforced under the project's [Code of Conduct](./CODE_OF_CONDUCT.md).

## Getting Started

Getting started with developing Rome takes three commands. You will only need Node v14 or above.

```bash
git clone https://github.com/rome/tools
cd tools
# TBD
```

No dependency installation step is required as we check in our `node_modules` folder that contains only a copy of TypeScript and some definitions.

### User files

If files specific to your local development environment should be ignored, please add these files to a global git ignore file rather than to a git ignore file within Rome.

You can find more information on this process [here](https://help.github.com/en/github/using-git/ignoring-files#configuring-ignored-files-for-all-repositories-on-your-computer).

## Website

The [Rome website](https://rome.tools/) is built with [Eleventy](https://www.11ty.dev/). To start a development server you can run the following commands:

```bash
cd website
npm install
npm start
```

## Checks


- `cargo lint` is a cargo alias that runs [`clippy'](https://github.com/rust-lang/rust-clippy) - rust official linter - under the hood;
- `cargo format` is a cargo alias that runs [`rust-fmt`](https://github.com/rust-lang/rustfmt) - rust official formatter - under the hood;
- `cargo test` will run the suite; make sure to run this command from the root of the project, so it will run the tests of all the internal crates;

### Generated files

If you work on some parser and you create new nodes or modify existing ones, will need to run a command to update some files that are auto-generated.

#### `cargo codegen grammar`

This command will update the syntax of the parsers.

The source is generated from the [`ungram` files](https://github.com/rome/tools/blob/main/xtask/codegen/js.ungram).

#### `cargo codegen test`


This command will create new tests for your parser. We currently have a neat infrastructure
where tests for parser are generated com inline comments found inside
the source code. Please read [the proper chapter for more information](#write-tests-for-a-parser)

It's strongly advised to **run this command before committing new changes**.

#### `cargo coverage`

This command will check and report parser conformance against different test suites.
We currently target the [Official ECMAScript Conformance Test Suite](https://github.com/tc39/test262) and
the [Typescript Test Suite](https://github.com/microsoft/TypeScript/tree/main/tests)

## Commit messages

Internally, the Rome team adheres as closely as possible to the [conventional commit specification](https://www.conventionalcommits.org/en/v1.0.0-beta.2/).
The following this convention encourages commit best-practices and facilitates commit-powered features like change log generation.

The following commit prefixes are supported:

- `feat:`, a new feature
- `fix:`, a bugfix
- `docs:`, a documentation update
- `test`, a test update
- `chore:`, project housekeeping
- `perf:`, project performance
- `refactor:`, refactor of the code without change in functionality

Below are examples of well-formatted commits:

```
feat(compiler): implement parsing for new type of files
fix: fix nasty unhandled error
docs: fix link to website page
test(lint): add more cases to handle invalid rules
```

### Creating pull requests

When creating a new pull request, it's preferable to use a conventional commit-formatted title, as this title will be used as the default commit message on the squashed commit after merging.

Here are some other scripts that you might find useful.

#### If you are a core contributor

If you are a core contributor, and you have access to create new branches
from the main repository (not a fork), use these comments to run specific workflows:

- `!bench_parser` benchmarks the parser's runtime performance and writes a comment with the results;
- `!bench_formatter` benchmarks the formatter runtime performance and writes a comment with the results;

#### Naming patterns

1. Forbid a concept

	```
	no<Concept>
	```

	When a rule's sole intention is to **forbid a single concept** - such as disallowing the use of `debugger` statements - the rule should be named using the `no` prefix. For example, the rule to disallow the use of `debugger` statements is named `noDebugger`.

1. Mandate a concept

	```
	use<Concept>
	```

 	When a rule's sole intention is to **mandate a single concept** - such as forcing the use of camel-casing - the rule should be named using the `use` prefix. For example, the rule to mandating the use of camel-cased variable names is named `useCamelCase`.

### Write parsers and parsing rules

To have a better understanding of our parsing infrastructure, please [read the in-depth section](/crates/rslint_parser/docs/authoring_parse_rules.md)

### Write tests for a parser

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


### Write tests for the formatter

We use [insta.rs](https://insta.rs/docs) for our snapshot tests, please make sure you read its documentation to learn the basics of snapshot testing.
You should install the companion [`cargo-insta`](https://insta.rs/docs/cli/) command to assist with snapshot reviewing.

To create a new snapshot test for JavaScript, create a new file to `crates/rome_formatter/tests/specs/js/`, e.g. `arrow_with_spaces.js`

```javascript
const foo     = ()    => {
	return bar
}
```

Files processed as modules must go inside the `module/` directory, files processed as script must go inside the 
`script/` directory.

Run the following command to generate the new snapshot (the snapshot tests are generated by a procedure macro so we need to recompile the tests):

```bash
touch crates/rome_formatter/tests/spec_tests.rs && cargo test -p rome_formatter formatter
```

For better test driven development flow, start the formatter tests with [`cargo-watch`](https://crates.io/crates/cargo-watch):

```bash
cargo watch -i '*.new' -x 'test -p rome_formatter formatter'
```

After test execution, you will get a new `arrow.js.snap.new` file.

To actually update the snapshot, run `cargo insta review` to interactively review and accept the pending snapshot. `arrow.js.snap.new` will be replaced with `arrow.js.snap`

Sometimes, you need to verify the formatting for different cases/options. In order to do that, create a folder with 
the cases you need to verify. If we needed to follow the previous example: 

1. create a folder called `arrow_with_spaces/` and move the JS file there;
2. then create a file called `options.json`
3. The content would be something like:
    ```json
    {
        "cases": [
            {
                "line_width": 120,
                "indent_style": {"Space": 4}
            }
        ]
    }
    ````
4. the `cases` keyword is mandatory;
5. then each object of the array will contain the matrix of options you'd want to test.
   In this case the test suite will run a **second test case** with `line_width` to 120 and `ident_style` with  4 spaces
6. when the test suite is run, you will have two outputs in your snapshot: the default one and the custom one

## VS Code Extension Development

To build the VS Code extension from source, navigate to the `editors/vscode` directory and run:

```bash
npm install
npm run build
```

This will create a `rome_lsp.vsix` which you can install into VS Code by running:

```bash
npm run install-extension
```

The Rome language server is the binary crate `rome_lsp` which can be built using `cargo build`.

Use the `"rome.lspBin"` VS Code setting to set the path to the executable:
```json
	"rome.lspBin": "/path/to/rome/target/debug/rome_lsp"
```

When performing any benchmarks for the language server, be sure to use a release build.
