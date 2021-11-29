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

#### `cargo xtask syntax`

This command will update the syntax of of the parsers.

#### `cargo xtask codegen`


This command will create new tests for your parser. We currently have a neat infrastructure
where tests for parser are generated com inline comments found inside
the source code. Please read [the proper chapter for more information](#write-tests-for-a-parser)


It's strongly advised to **run this command before committing new changes**.

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

Now you need to run `cargo xtask codegen` and the task will actually generate this file for you.

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

Run the command `cargo xtask codegen` and you will see a new file called
`feature_name.js` inside the `test_data/inline/err` folder.

The content of this file will be:

```js
let a = {  : "" }
let b = { new_feature :  }
```

Now run the command `env UPDATE_EXPECT=1 cargo test` which will tell the
test suite to generate and update the `.rast` files.

If tests that are inside the `ok/` folder fails or if tests that are inside the `err/`
folder don't fail, the whole test suite will fail.
