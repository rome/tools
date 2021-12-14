# Authoring Parse Rules

This is a short, or not so short, guide to implement parse rules using the Rome parser infrastructure.

## Naming
The convention is to prefix your parse rule with `parse_` and then use the name defined in the grammar file.

For example, `parse_for_statement` or `parse_expression`.

## Signature
Most parse rules take a `&mut` reference to the parser as their only parameter and return a `ParsedSyntax`.

```rust
fn parse_rule_name(&mut: Parser) -> ParsedSyntax {}
```

You're free to add additional parameters to your function if needed. There are rare cases where you want to consider returning `ConditionalParsedSyntax` as explained in [conditional syntax](#conditional-syntax)


## Parsing a single node

Let's assume you want to parse the JS `if` statement:

```
JsIfStatement =
 if
 (
 test: JsAnyExpression
 )
 consequent: JsBlockStatement
 else_clause: JsElseClause?
```

### Presence Test

Now, the parsing function must first test if the parser is positioned at an `if` statement and return `Absent` if that's not the case.

```rust
if !p.at(T![if]) {
 return ParsedSyntax::Absent;
}
```

Why return `ParsedSyntax::Absent`? The function must return `ParsedSyntax::Absent` if the rule can't predict by the next token(s) if they form the expected node or not. Doing so allows the calling rule to decide if this is an error and perform an error recovery if necessary.  The second reason is to ensure that the rule doesn't return a node where all children are missing.

Your rule implementation may want to consider more than just the first child to determine if it can parse at least some of the expected children.
For example, the if statement rule could test if the parser is located at an `else` clause and then create an `if` statement where all children are missing except the `else` clause:

```rust
if !p.at(T![if]) && !p.at(T![else]){
  return Absent
}
```

Your implementation can also call into another parsing rule if the first child is a node and not a token.

```rust
let assignment_target = parse_assignment_target(p);

if assignment_target.is_absent() {
  return Absent;
}

let my_node = assignment_target.precede_or_missing();
```

But be careful with calling other rules. Your rule mustn't progress the parser - meaning that it can't
advance in the parsing process and consume tokens - if it returns `Absent`.


### Parse children
The parse rules will guide you in how to write your implementation and the parser infrastructure provides the following convenience APIs:

* Optional token `'ident'?`: Use `p.eat_optional(token)`. It eats the next token if it matches the passed-in token. Adds a missing marker if the token isn't present in the source code.
* Required token `'ident'`: Use`p.expect_required(token)`. It eats the next token if it matches the passed-in token. 
It adds an `Expected 'x' but found 'y' instead` error and a missing marker if the token isn't present in the source code.
* Optional node `body: JsBlockStatement?`: Use`parse_block_statement(p).or_missing(p)`. It parses the block if it is present in the source code and adds a missing marker if it isn't.
* Required node `body: JsBlockStatement`: Use `parse_block_statement(p).or_missing_with_error(p, error_builder)`:
it parses the block statement if it is present in the source code and adds a missing marker and an error if not.

Using the above-described rules result in the following implementation for the `if` statement rule.

```rust
fn parse_if_statement(p: &mut Parser) -> ParsedSyntax {
 if !p.at(T![if]) {
  return Absent;
 }

 let m = p.start();

 p.expect_required(T![if]);
 p.expect_required(T!['(']);
 parse_any_expression(p).or_missing_with_error(p, js_parse_errors::expeced_if_statement);
 p.expect_required(T![')']);
 parse_block_statement(p).or_missing_with_error(p, js_parse_errors::expected_block_statement);
// the else block is optional, so we mark it as "missing" in case it's absent
 parse_else_clause(p).or_missing();

 Present(m.complete(p, JS_IF_STATEMENT));
}
```

Hold on, what are these *missing* markers? Rome's AST facade uses fixed offsets to retrieve a particular child from a node. 
For example, the 3rd child of the if statement is the condition. However, the condition would become the second element 
if the opening parentheses `(` isn't present in the source text. That's where missing elements come into play. 
Missing elements (added by calling `p.missing()`) represent placeholders for syntax that isn't present in the source text to guarantee that the children always appear in the same order.

## Parsing Lists & Error Recovery

Parsing lists is different from parsing single elements with a fixed set of children because it requires looping until 
the parser reaches a terminal token (or the end of the file).

You may remember that `parse_*` methods shouldn't progress parsing if they return `Absent`. 
Not progressing the parser is problematic inside `while` loops because it inevitably results in an infinite loop.

That's why you must do error recovery when parsing lists. Luckily, the parser comes with the infrastructure to make error recovery a piece of cake.
The general structure for parsing a list is (yes, that's something the parser infrastructure should provide for you):


Let's try to parse an array: 

```js
[ 1, 3, 6 ]
```

We will use  `ParseSeparatedList` in order to achieve that

```rust
struct ArrayElementsList;

impl ParseSeparatedList for ArrayElementsList {
    type ParsedElement = CompletedMarker;

    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax<Self::ParsedElement> {
        parse_array_element(p)
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        p.at_ts(token_set![T![default], T![case], T!['}']])
    }

    fn recover(
        &mut self,
        p: &mut Parser,
        parsed_element: ParsedSyntax<Self::ParsedElement>,
    ) -> parser::RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(JS_UNKNOWN_STATEMENT, STMT_RECOVERY_SET),
            js_parse_error::expected_case,
        )
    }
};
```

Let's run through this step by step:

```rust
parsed_element.or_recover(
    p,
    &ParseRecovery::new(JS_UNKNOWN_STATEMENT, STMT_RECOVERY_SET),
    js_parse_error::expected_case,
)
```

The `or_recover` performs an error recovery if the `parse_array_element` method returns `Absent`; 
there's no array element in the source text. 

The recovery eats all tokens until it finds one of the tokens specified in the `token_set`, 
a line break (if you called `enable_recovery_on_line_break`) or the end of the file. 

The recovery doesn't throw the tokens away but instead wraps them inside a `UNKNOWN_JS_EXPRESSION` node (first parameter). 
There exist multiple `UNKNOWN_*` nodes. You must consult the grammar to understand which `UNKNOWN*` node is supported in your case.

> You usually want to include the terminal token ending your list, the element separator token, and the token terminating a statement in your recovery set.


Now, the problem with recovery is that it can fail, and there are two reasons:

- the parser reached the end of the file;
- the next token is one of the tokens specified in the recovery set, meaning there is nothing to recover from;

In these cases the `ParseSeparatedList` and `ParseNodeList` will recover the parser for you.

## Conditional Syntax

The conditional syntax allows you to express that some syntax may not be valid in all source files. Some use cases are:

* syntax that is only supported in strict or sloppy mode: for example, `with` statements is not valid when a JavaScript file uses `"use strict"` or is a module;
* syntax that is only supported in certain file types: Typescript, JSX, modules;
* syntax that is only available in specific language versions: experimental features, different versions of the language e.g. (ECMA versions for JavaScript);

The idea is that the parser always parses the syntax regardless of whatever it is supported in this specific file or context. 
The main motivation behind doing so is that this gives us perfect error recovery and allows us to use the same code regardless of whether the syntax is supported.

However, conditional syntax must be handled because we want to add a diagnostic if the syntax isn't supported for the current file, and the parsed tokens must be attached somewhere.

Let's have a look at the `with` statement that is only allowed in loose mode/sloppy mode:

```rust
fn parse_with_statement(p: &mut Parser) -> ParsedSyntax {
 if !p.at(T![with]) {
  return Absent;
 }

 let m = p.start();
 p.bump(T![with]); // with
 parenthesized_expression(p).or_missing_with_error(p, js_errors::expected_parenthesized_expression);
 parse_statement(p).or_missing_with_error(p, js_error::expected_statement);
 let with_stmt = m.complete(p, JS_WITH_STATEMENT);

 let conditional = StrictMode.excluding_syntax(p, with_stmt, |p, marker| {
  p.err_builder("`with` statements are not allowed in strict mode")
   .primary(marker.range(p), "")
 });

 conditional.or_invalid_to_unknown(p, JS_UNKNOWN_STATEMENT)
}
```

The start of the rule is the same as for any other rule. The exciting bits start with

```rust
let conditional = StrictMode.excluding_syntax(p, with_stmt, |p, marker| {
 p.err_builder("`with` statements are not allowed in strict mode")
  .primary(marker.range(p), "")
});
```

The `StrictMode.excluding_syntax` creates a conditional syntax that is invalid in strict mode and adds an error if the parser is currently in strict mode.

You can convert the `ConditionalParsedSyntax` to a regular `ParsedSyntax` by calling `or_invalid_to_unknown`, which wraps the whole parsed `with` statement in an `UNKNOWN` node if the parser is in strict mode and otherwise returns the unchanged `with` statement.

What if there's no `UNKNOWN` node matching the node of your parse rule? You must then return the `ConditionalParsedSyntax` without making the `or_invalid_to_unknown` recovery. It's then up to the caller to recover the potentially invalid syntax.


## Summary

* Parse rules are named `parse_rule_name`
* The parse rules should return a `ParsedSyntax` or `ConditinalParsedSyntax`
* The rule must return `Present` if it consumes any token and, therefore, can parse the node with at least some of its children.
* It returns `Absent` otherwise and must not progress parsing nor add any errors.
* Lists must perform error recovery to avoid infinite loops.
* Consult the grammar to identify the `UNKNOWN` node that is valid in the context of your rule.
