# xtask

This crate contains local commands used to auto-generate source code.

The project uses [`ungrammar`](https://github.com/rust-analyzer/ungrammar) to define the syntax of the language.

`ungrammar` uses a DSL to define and parse the grammar of a language.

Once the library parses the DSL files, some custom logic generates the AST APIs.

## Conventions to create syntax

Here's a list of internal conventions that we follow to write grammar:

- use the prefix `manual__` to mark methods that are implemented manually;

```
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
