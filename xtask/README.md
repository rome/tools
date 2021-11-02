# xtask

This crate contains local commands used to auto-generate source code.

The project uses [`ungrammar`](https://github.com/rust-analyzer/ungrammar) to define the syntax of the language.

`ungrammar` uses a DSL to define and parse the grammar of a language.

Once the library parses the DSL files, some custom logic generates the AST APIs.

## Conventions to create syntax

Here's a list of internal conventions that we follow to write grammar:

1. use the prefix `manual__` to mark methods that are implemented manually;

	```text
	MyDeclaration = manual__decl:Body
	```

	If you define like this, it means that somewhere in your code you will create something like:

	```rust
	impl MyDeclaration {
		fn decl(self) -> Option<Body> {
			// custom logic goes here
		}
	}
	```

2. unions of tokens have to have a label. In this way, the code generation can handle it properly and generate a better AST.

	```text
	BinExpr = left: Expr op: ('+' | '-' | '*') right: Expr
	```

	This will generate the correct implantation:

	```rust
	impl BinExpr {
		fn op(self) -> Option<SyntaxToken> {
			// custom logic goes here
		}
	}
	```
