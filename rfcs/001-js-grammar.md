
## Codegen extensions

* **Must**: Add support for labels so that we can influence how the child accessors are named. For example, we want to name the left and right expressions of `BinaryExpression` `left` and `right` and not `expression` and `expression. Ungrammar supports the `label: Type` syntax. All we need to do is to respect the naming when generating the fields.
* **Must**: Support inline token-union types. For example, the binary expression operator can be any of `==, +, -, ...`. That's why it's defined as `operator: ( '==' | '+' | '-' ...)`. The source gen should only generate a single field for the operator that returns a `SyntaxToken`
* **Must**: Generate different return types for mandatory/optional children.
* **Must/Wish**: It's convenient to define unions in respect of other unions. For example, `JsClassMemberName = JsObjectMemberName | JsPrivateClassMemberName` where `JsObjectMemberName` is an union type as well. This is convenient to maintain the grammar but makes the Facade more awkward to use because it requires two matches: first on the outer union and then on the inner union. We can avoid this if we flatten unions inside of the source gen and automatically generate `From<InnerUnion>` implementations.
* **Wish**: Automatically strip the `Js` extensions from field names to reduce the need for explicit labels. For example, `ExpressionStatement` must use an explicit label for its inner expression so that it isn't named `js_expression`.
* **Wish**: Generation of a union type over all nodes/tokens of a language, `JsElement = ..., JsNode = ..., JsToken = ...`

## Proposed Changes

### JS Prefixing
* Prefix nodes  with `Js` to avoid conflicts with other languages
* Prefix tokens with `Js` if they are not representing a specific character sequence. For example, prefix `js_string` because different languages use different quotes for strings but don't prefix `;` or `true` because these are "pure" character sequences. `true` might be somewhat controversial because different languages use different casing `TRUE` vs `true` vs `True`. I don't think the casing is relevant. All these represent a `true` token and the corresponding parser shouldn't generate `TRUE` for a language expecting `true` and each language-impl can provide helpers to build common tokens.


### Separated sequences

It's common in languages to have a sequence of elements that all are separated by a specific separator. Examples in JavaScript where elements are comma separated are:

* Array elements
* Object properties
* Import/Export specifiers
* ...

There are two ways how we can model this. I'll use a simplified version of JS arrays that ignores array holes.

**Wrap elements in a node**

One option is to create a new node representing an `ArrayElement` that contains the value and an optional trailing comma.

```
JsArray = '[' elements: JsArrayElement* ']

JsArrayElement =
  value: JsArrayElementValue
  trailing_comma: ','?

JsArrayElementValue = JsExpression | JsSpread
```

Creating a new node has a few downsides:

* Undesired nesting
* Naming becomes tricky if the element values are over a union type. How to avoid the name clash of the union type and the `Element` type?
* Repetitive grammar: JS defines multiple element-lists that are separated by commas.
* Mutating the elements requires extra care to ensure that the appropriate number of commas are in (the right) place

I saw that Roslyn' uses a [`SeparatedList`](https://docs.microsoft.com/en-us/dotnet/api/microsoft.codeanalysis.separatedsyntaxlist-1). It's just a nice view over a list of children and I think we can adopt a similar approach in the AST Facade by returning `AstSeparatedChildren` for such element lists.
`AstSeparatedChildren` is a view over a range of children that alternate between element and separator. It may provide the following methods:

* `nodes() -> Iterable<Result<T>>`: Returns the nodes inside of the list. The return type is a result in case a node is missing between two separators
* `separators() -> Iterable<SyntaxToken>`: returns all the separators. There should be no need for an `Option` or `Result` because the parser never starts a new element if a separator is missing
* `elements() -> Iterable<(Result<T>, Option<SyntaxToken>)>`: Returns a list of elements with their separator, for example for use inside of the formatter. The separator is optional because the last element may or may not have a trailing separator.

We could add additional methods for mutating a separated list, querying a specific `node`/`separator`/`element` etc.

I believe this gives us the same advantages as having distinct elements without the repetition:

```
JsArray = '[' elements: (JsArrayElement  (',' JsArrayElement )* ','?) ']

JsArrayElement = JsExpression | JsSpread
```

We should even be able to automatically generate the proper return type on the field by testing whenever we see the `(T (',' T)* ','?)` pattern inside of the grammar, something that [rust-analyzer does too](https://github.com/rust-analyzer/rust-analyzer/blob/dfa355b4313a7ea3eb5a262ee9f1da71f50884d5/crates/syntax/src/tests/sourcegen_ast.rs#L676) already does.

## Open Questions

### Member vs Property naming

Should we call object and class members `Members` or `Properties`?

* `MemberExpression` has an `object` and `property` field today which feels inconsistent. Shouldn't property be called member or should it be a `PropertyExpression`
* A property can be a value property, method, constructor, or a getter/setter. How to we prevent that we don't end up with property property?
* The term member is commonly used in other languages

The main downside I see is that property is a commonly used term in the JS ecosystem. Changing property to member could be an entry barrier for people familiar with the term property.

### AST conformance

How strict should our AST API be?

* Should directives be allowed into `BlockStatements` or should we have separate `FunctionBody` and `BlockStatement` nodes where only `FunctionBody` allows directives?
* Should a `script` allow module statements inside of the body?
* Should a `module` have an explicit field for `import` statements at the top?
* Should assignment expression allow left-hand sides that aren't valid assignment targets?

There's probably no correct answer to this question but we can favour one over the other.

A strict AST has the benefit that the mutation API prevents users from constructing invalid trees but it comes at the cost that our parser must fall back to `Unknown*`  nodes more often and a more complex AST.

I would recommend being more lenient so that the parser can flag the use of `import/export` inside a script but we can still provide auto-completion on imports/exports before the user converts the script to a module. But it's probably something we must decide case by case but would love to hear your thoughts.

### Node granularity

Having fine granular nodes has the advantage that the API allows querying for a very specific node when, for example, using `node.descendants::<PreIncrementExpression>()`.

Having more coarse-grained nodes on the other hand has the advantage that it's easier to implement common behaviour that, for example, applies to all Binary expressions.

I guess, this isn't a specific question, but we should think about if we favour fine granular nodes, being even as extreme as having a different node for each `BinaryExpression` operator.


## Ungrammar extension proposals

Add support for `///` comments that we can use to document nodes and tokens.
