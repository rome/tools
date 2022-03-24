# Contributing

We can use help in a bunch of areas and any help is appreciated. Our [GitHub issues](https://github.com/rome/tools/issues) serve as a place for any discussion, whether it's bug reports, questions, project direction etc. As the project grows this policy may change.

Our [Discord server](https://discord.gg/rome) is open for help and more adhoc discussion. All activity on the Discord is still moderated and will be strictly enforced under the project's [Code of Conduct](./CODE_OF_CONDUCT.md).

## Getting Started

Building this project requires a `stable` Rust toolchain, which can be installed using [rustup](https://www.rust-lang.org/tools/install).

Clone the repository and navigate to the `tools` directory:
```bash
git clone https://github.com/rome/tools
cd tools
```
Compile all packages and dependencies:
```bash
cargo build
```
Rome can be used via the `rome` bin in the `rome_cli` package:
```bash
cargo run --bin rome -- --help
```

Rome can be used as a language server by following the instructions below.

## Language Server and VS Code Extension Development

The Rome language server is the binary crate `rome_lsp` which can be built using:
```
cargo build --bin rome_lsp
```
If benchmarking the language server, be sure to build with the `--release` flag.

The VS Code extension can be installed from the [Marketplace](https://marketplace.visualstudio.com/items?itemName=rome.rome) and can be used with a development build of the language server by setting the `"rome.lspBin"` VS Code setting to the path of the binary:
```json
	"rome.lspBin": "/path/to/rome/target/debug/rome_lsp"
```

To instead build the VS Code extension from source, navigate to the `editors/vscode` directory and run:

```bash
npm install
npm run build
```

This will create a `rome_lsp.vsix` which you can install into VS Code by running:

```bash
npm run install-extension
```

The `"rome.lspBin"` VS Code setting will still need to be set as described above.

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

The test suites are included as git submodules and can be pulled using:
```bash
git submodule update --init --recursive
```

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

To have a better understanding of our parsing infrastructure, please [read the in-depth section](/crates/rome_js_parser/docs/authoring_parse_rules.md)

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


### Formatter

- [write tests for the formatter](/crates/rome_formatter/docs/write_tests.md)
- [implement the formatter](/crates/rome_formatter/docs/implement_the_formatter.md)


### Versioning

We follow the specs suggested by [the official documentation](https://code.visualstudio.com/api/working-with-extensions/publishing-extension#prerelease-extensions):

Odd minor versions are dedicated to pre-releases, e.g. `*.5.*` .
Even minor versions are dedicated to official releases, e.g. `*.6.*`.
