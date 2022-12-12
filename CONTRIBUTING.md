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

The Rome language server is the binary crate `rome` which can be built using the command:

```bash
cargo build --bin rome
```
If benchmarking the language server, be sure to build with the `--release` flag.

The VS Code extension can be installed from the [Marketplace](https://marketplace.visualstudio.com/items?itemName=rome.rome) and can be used with a development build of the language server by setting the `"rome.lspBin"` VS Code setting to the path of the binary:

```json
	"rome.lspBin": "/path/to/rome/target/debug/rome"
```

Please note that Windows disallows modifying an executable while it's running,
meaning you won't be able to recompile the Rome binary once the extension was activated in your editor.

The server is spawned as a background daemon, and continues to run even after the editor is closed.

To stop the running daemon instance use the `rome stop` command, with the editor closed as the extension
will try to restart it otherwise.

To build the VS Code extension from source, navigate to the `editors/vscode` directory and run:

```bash
npm install
npm run build
```

This will create a `rome_lsp.vsix` which you can install into VS Code by running:

```bash
npm run install-extension
```

The `"rome.lspBin"` VS Code setting will still need to be set as described above.

When the extension is running, it will connect to a daemon server - or it will bootstrap one.

When you apply changes to the binary, you need to do two things:
- compile the binary
- kill the daemon process, so you can spawn a new server session
with the new changes

When the daemon is running, it's possible to inspect its logs in the folder `rome-logs`, placed
in the temporary folder of the operative system.


### User files

If files specific to your local development environment should be ignored, please add these files to a global git ignore file rather than to a git ignore file within Rome.

You can find more information on this process [here](https://help.github.com/en/github/using-git/ignoring-files#configuring-ignored-files-for-all-repositories-on-your-computer).

## Node.js development

The npm module npm/rome contains Rome's Node JS API that supports different backends:
- `wasm-nodejs` (WebAssembly)
- `backend-jsonrpc` (Connection to the daemon)

For testing and developing, you need to build these packages, following the steps:
1. install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) globally;
2. run the `build` command inside the package `backend-jsonrpc`;
3. run the `build:wasm-node-dev` command inside the package `js-api` (folder `npm/js-api`);
4. run `pnpm i` inside the package `js-api` (folder `npm/js-api`), this will link the WebAssembly bindings and the
JSON-RPC bindings;

The tests are run against the compiled files, which means that you need to run the
`build` command after you implemented features/bug fixes.

## Website

The [Rome website](https://rome.tools/) is built with [Eleventy](https://www.11ty.dev/). To start a development server you can run the following commands:

```bash
cd website
npm install
npm start
```

## Checks


- `cargo lint` is a cargo alias that runs [`clippy`](https://github.com/rust-lang/rust-clippy) - rust official linter - under the hood;
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

#### `cargo codegen analyzer`

This command will detect linter rules declared in the `analyzers` and `assists` directories in `rome_analyze`, regenerate the index modules `analyzers.rs` and `assists.rs` to import these files, and update the registry builder function in `registry.rs` to include all these rules.
It will also regenerate the configuration of the rules.

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
- `test:`, a test update
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

Please use the template provided.

If you PR requires some update on the website (new features, breaking changes, etc.), a new follow-up
PR should be created against the "release" PR. If you can't create a new PR, please let the team know,
the template should help to give all the information to the team.

The team will prepare a new PR after each new release, just search for "release" among the opened pull requests.

Here are some other scripts that you might find useful.

#### If you are a core contributor

If you are a core contributor, and you have access to create new branches
from the main repository (not a fork), use these comments to run specific workflows:

- `!bench_parser` benchmarks the parser's runtime performance and writes a comment with the results;
- `!bench_formatter` benchmarks the formatter runtime performance and writes a comment with the results;
- `!bench_analyzer` benchmarks the analyzer runtime performance and writes a comment with the results;

### Analyzers and lint rules

To know the technical details of how our analyzer works, how to create a rule and how to write tests, please check our [internal
documentation page](https://rustdocs.rome.tools/rome_analyze/index.html)

### JavaScript Parser

To know the technical details of how our JavaScript works and how to write test, please check our [internal
documentation page](https://rustdocs.rome.tools/rome_js_parser/index.html)

### Formatter

To know the technical details of how our formatter works and how to write test, please check our [internal
documentation page](https://rustdocs.rome.tools/rome_js_formatter/index.html)

### Versioning

We follow the specs suggested by [the official documentation](https://code.visualstudio.com/api/working-with-extensions/publishing-extension#prerelease-extensions):

Odd minor versions are dedicated to pre-releases, e.g. `*.5.*` .
Even minor versions are dedicated to official releases, e.g. `*.6.*`.

### Playground

- [run the playground locally](/website/playground/README.md)

### Snapshot tests

Internally, we use [`insta`](https://insta.rs/) for snapshot tests. This means that you
follow their [installation instructions](https://insta.rs/docs/cli/) to update/accept
the new snapshot tests.

### Using just

A lot of the commands above are mor easily accessible using our [Just](https://just.systems/man/en/) recipes. For example:

### Install just

You can install `just` using cargo:

```shell
cargo install just
```

Or, using different methods, like explained in their [documentation](https://just.systems/man/en/chapter_4.html).

It's advised to install `just` using a package manager, so
you can run `just` as a binary.

### Usage

```ignore
❯ just
just --list -u
Available recipes:
    codegen
    documentation
    new-lintrule path name
    test-lintrule name
    check-ready
```

All the necessary `codegen` can be called using

```ignore
> just codegen
```

After all changes are done, the code can be checked if is ready to be pushed with

```ignore
> just check-ready
```
