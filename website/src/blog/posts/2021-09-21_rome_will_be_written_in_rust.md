---
title: Rome will be written in Rust 🦀
description: Building a better foundation for the future of JavaScript and Web tooling
author_name: Jamie Kyle
author_url: https://twitter.com/buildsghost
author_avatar: /img/blog/jamie-avatar.jpg
date: 2021-09-21
tags:
	- post
  - update
  - rust
permalink: /blog/2021/09/21/rome-will-be-rewritten-in-rust.html
layout: layouts/blog.liquid
social-image: social-logo-rust.png
cover-image: rust-cover.jpg
---

Rome started off written in JavaScript because that is the language of choice for our team and it made it easier for others in the community to join as contributors. We love JavaScript and TypeScript (and HTML and CSS) at Rome, and we want to build the very best tooling possible for these languages. For a number of reasons, we've decided that Rust will provide a better foundation for this tooling.

We are also taking the opportunity to explore fundamental shifts in the architecture of Rome. These changes give us more flexibility and will allow us to build tooling JavaScript and the web has not had before.

<!-- DESCRIPTION_END -->

## Why write Rome in Rust?

Many others have communicated the performance, memory, and safety benefits of Rust before us - let's just say everyone who has ever said Rust is good is correct. However, our biggest concern was our own productivity.

It was initially pretty hard to imagine a small team of JavaScript developers getting ramped up in a new language with enough expertise to build complex language tooling and be productive in a matter of weeks. After some prototyping however, we quickly realized we might actually be more productive in Rust.

One of the major early decisions in Rome was not to use any third-party dependencies - we even wrapped Node APIs with our own. This decision was based on a desire to tightly control all of the code inside of Rome so that we could make guarantees about performance, memory usage, and correctness/type-safety.

However, many of those concerns are addressed in Rust and its community:

1. **Fewer tradeoffs for third-party dependencies**
*Most JavaScript/npm packages have to balance a lot of different concerns for many different kinds of users. They are forced to make tradeoffs around code size and performance that we aren't interested in. Rust crates, on the other hand, are generally making tradeoffs that much closer align to our needs.*
2. **Correctness is built-in to the standard library and in most popular crates**
*We were creating our own APIs that focused on correctness instead of using third-party JavaScript dependencies. Rust, and its community, places a lot more focus on correctness without paving over edge cases that we need to worry about for Rome.*
3. **Trait/Module system allows us to make better use of dependencies**
*Rust's trait system is a powerful way to [create abstractions over any piece of data without overhead](https://blog.rust-lang.org/2015/05/11/traits.html). It allows us to deeply integrate third party libraries. It also allows libraries to create APIs that are much more incremental, safely exposing more surface area without creating a need to make breaking changes.*

We realized that the reasons we avoided third party dependencies wouldn't apply when working in Rust. Being able to build off of high quality dependencies without making tradeoffs makes us more productive and will lead to a better, faster Rome.

## What are we changing?

When we started prototyping in Rust and revisiting a lot of our fundamental design decisions, it also provided us with an opportunity to take another look at our architecture. Quickly, we realized we wanted to make some very large changes.

If you've ever spent time [learning how compilers work](https://github.com/jamiebuilds/the-super-tiny-compiler), you've likely heard something like:

1. *Source Code* → **Lexical Analysis** → *Tokens*
2. *Tokens* → **Syntactic Analysis** → *Abstract Syntax Tree*
3. *Abstract Syntax Tree* → **Various Transformations** → *Some Intermediate Representation*
4. *Some Intermediate Representation* → **Code Generation** → {*Machine,Byte,Source} Code*

That is a good high-level mental model for how compilers work, but most compilers get much more complex than that.

Very quickly you are going to want a couple of things:

- Incrementally build projects while the developer is making code changes.
- Request information about some code before the compiler has completed work.
- Continue to provide feedback even while the code contains syntax errors.

For the JavaScript and Web communities, these responsibilities have often been split between many different tools, which causes everyone to implement the same things in slightly different ways over and over. Rome wants to be all of this tooling at once, so we need a foundation that will work for all these different tools.

We looked at how other, newer compilers have solved these problems. One of the key problems we soon realized was our approach to Abstract Syntax Trees (ASTs).

### What's the deal with Abstract Syntax Trees?

Abstract Syntax Trees (generally, and more specifically in our original design):

1. Must represent a complete and valid program, and/or are consumed with the assumption that they represent a complete and valid program.
2. Must be produced all at once, and must be rebuilt from scratch when source changes.
3. Must stay valid and complete after each stage of transformations, and/or cannot be printed with invalid or incomplete syntax.
4. Do not include enough information to construct the original source, and/or cannot partially reformat portion.

```ruby
# `let value = 42;`
Node(Program,
  body: Node(StatementList, statements: [
    Node(VariableDeclaration, kind: "let", declarations: [
			Node(VariableDeclarator,
				id: Node(BindingIdentifier, name: "value")
				initializer: Node(NumericLiteral, value: 42)
      )
		])
  ])
)
```

If we compare this to the raw lexical token stream, we see a different set of tradeoffs. We have the capability to represent incomplete/invalid programs. We can make mutations without destroying source information elsewhere. However, working directly with tokens would push all of the syntactic complexity around the codebase.

```ruby
# `let value = 42;`
Token(LetKeyword, "let")
Token(Whitespace, " ")
Token(Identifier, "value")
Token(Whitespace, " ")
Token(Equals, "=")
Token(Whitespace, " ")
Token(NumericLiteral, "42")
Token(Semicolon, ";")
```

Another option between the two is a Concrete Syntax Tree. This will often look like an Abstract Syntax Tree but will contain enough information to reconstruct the source.

```ruby
# `let value = 42;`
Node(Program,
  body: Node(StatementList, statements: [
    Node(VariableDeclaration,
      kind: Token(LetKeyword, trailing_trivia: Token(Whitespace, " "))
      declarations: [
				Node(VariableDeclarator,
					id: Node(BindingIdentifier,
						name: "value",
						trailing_trivia: Token(Whitespace, " ")
					)
					equals: Token(Equals, trailing_trivia: Token(Whitespace, " "))
					initializer: Node(NumericLiteral, value: 42)
	      )
			]
			semicolon: Token(Semicolon)
		)
  ])
)
```

However, this specific type of Concrete Syntax Tree (CST) inherits a lot of the same problems as an AST - It still requires valid syntax and it's easy to lose track of the original source text when making edits to the tree.

To reproduce the benefits of both lexical tokens and a syntactic tree, we need a different data structure for our CST that meets all of the following:

- Must be a tree where the nodes of the tree represent abstract syntactic structure.
- Must contain lexical tokens that represent the literal source text.
- Must be able to represent invalid/incomplete programs while preserving both the abstract syntactic structure and lexical tokens representing the literal source text.
- Must not be fragile to making edits in the tree.

The resulting data structure is a tree with syntactic branches, but where each branch has a consistent structure:

```fsharp
type Token(SyntaxKind, source_text: String)
type Node(SyntaxKind, children: List<Node | Token>)
```

Using this structure, you can traverse through the tree and print each of the tokens in order, and produce the exact source of the program.

```ruby
# `let value = 42;`
Node(VariableDeclaration, children: [
	Token(LetKeyword, "let"),
	Token(Whitespace, " "),
  Node(VariableDeclarator, children: [
		Node(BindingIdentifier, children: [
			Token(Identifier, "value"),
			Token(Whitespace, " "),
    ]),
		Token(Equals, "="),
		Token(Whitespace, " "),
		Node(NumericLiteral, children: [
			Token(NumericLiteral, "42"),
    ]),
  ]),
	Token(Semicolon, ";"),
])
```

This encodes both lexical and syntactic information in the same tree. From a node, we can tell exactly where we are in the syntax, and from its tokens we can tell exactly how the source code was written.

To make this easy to use, we'll also wrap the tree in another API that looks a lot like our original AST. This code will feel very familiar to anyone whose ever worked with an AST, but behind the scenes it's continuously checking the current node and its children to ensure we're only receiving valid and complete syntax.

```rust
fn visitor(node: Node) -> Option<_> {
	// checks if the current node is a `VariableDeclarator` and returns if it's not
	let variable_declarator = node.cast::<VariableDeclarator>()?;

	// checks if the variable_declarator has a valid id field and returns if not
	let id = variable_declarator.id()?;

	// ...
}
```

Together, our CST and this AST-like API allow us to easily represent invalid or incomplete programs, even placing errors directly into the tree:

```fsharp
type Error(String)
type Token(SyntaxKind, source_text: String)
type Node(SyntaxKind, children: List<Node | Token | Error>)
```

```ruby
# `let value =`
Node(VariableDeclaration, children: [
	Token(LetKeyword, "let")
	Token(Whitespace, " ")
  Node(VariableDeclarator, children: [
		Node(BindingIdentifier, children: [
			Token(Identifier, "value")
			Token(Whitespace, " ")
    ])
		Token(Equals, "=")
    Error("Unexpected EOF") # << our program ends too early
  ])
])
```

We can still traverse this tree, or even mutate it, as long as we don't try to inspect the error in it. Our visitor from before is only checking for the `id` field in our `VariableDeclarator` so it is able to run successfully without encountering the error. If it did encounter the error, we would return early and know that the visitor couldn't complete.

This particular type of CST and the API over it is known as a [Red-Green Tree](https://ericlippert.com/2012/06/08/red-green-trees/), coined by the C#/Roslyn compiler team. And it's been adopted by a number of different languages:

- [C#/Visual Basic/Roslyn](https://docs.microsoft.com/en-us/dotnet/csharp/roslyn-sdk/syntax-visualizer)
- [Rust Analyzer](https://github.com/rust-analyzer/rust-analyzer/blob/master/docs/dev/syntax.md)
- [Swift](https://github.com/apple/swift/tree/main/lib/Syntax)
- [RSLint for JavaScript](https://rslint.org/dev/untyped-trees.html)
- *(Please let us know if you are working on others!)*

Taking the time to explore Rust presented us with the opportunity to identify the benefits of implementing this CST. Consequently, we have drastically changed our approach to the creation of Rome and the types of problems it will have the capability of solving. Using this as a foundation, we can provide a world-class code editor experience that was impossible before.

## So what's next?

We have decided to move forward with a Rust-powered Rome. We've spent the last several weeks experimenting and prototyping, and found a lot to be excited about for the future. We're reaching the end of the prototype phase, and we'll be working out in the open to turn this into reality.

We're building a foundation for the future of JavaScript and Web tooling - tackling some of the largest challenges that these communities have faced for many years.

If you're a Rust developer and you're excited to get involved, we are currently [hiring for senior developers experienced in both Rust and compiler/language tooling](https://rome-tools-inc.breezy.hr/p/cf7ddbd89110).
