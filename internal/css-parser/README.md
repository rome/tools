# CSS Parser

This parser tries to mirror as much as possible the official spec.

The tokenization follows 100% the official spec.

Features that are not listed below are probably missing from the parser, being worked or
missing from the documentation. Please open a discussion
if you think it's worth implementing it.

## Features supported

Supported features are those features that have dedicated parsing and AST nodes. Having such information
inside a parser, allows Rome's compiler to create better lint rules and formatting.

### Values

- [Number](https://www.w3.org/TR/css-values-3/#numbers)
- [Dimension](https://www.w3.org/TR/css-values-3/#dimensions)
- [Percentage](https://www.w3.org/TR/css-values-3/#percentage-value)
- [String](https://www.w3.org/TR/css-values-3/#strings)
- [URL](https://www.w3.org/TR/css-values-3/#urls)

### Functions

- [`var()`](https://www.w3.org/TR/css-variables/#funcdef-var)
- [`calc()`](https://www.w3.org/TR/css-values-3/#calc-syntax)

### Selectors

- [type selector](https://www.w3.org/TR/selectors-4/#type-selectors)
- [universal selector](https://www.w3.org/TR/selectors-4/#the-universal-selector)
- [pseudo class](https://www.w3.org/TR/selectors-4/#pseudo-classes)
- [pseudo elements](https://www.w3.org/TR/selectors-4/#pseudo-elements)
- [attribute selector](https://www.w3.org/TR/selectors-4/#attribute-selectors)
- [descendant combinator](https://www.w3.org/TR/selectors-4/#descendant-combinators)
- [child combinator](https://www.w3.org/TR/selectors-4/#child-combinators)
- [next sibling combinator](https://www.w3.org/TR/selectors-4/#adjacent-sibling-combinators)
- [sibling combinator](https://www.w3.org/TR/selectors-4/#general-sibling-combinators)

### At rules

- [`@media`](https://www.w3.org/TR/mediaqueries-4/) - range text not yet supported
- [`@keyframe`](https://www.w3.org/TR/css-animations-1/#keyframes)
- [`@page`](https://www.w3.org/TR/css-page-3/#syntax-page-selector)
- [`@supports`](https://drafts.csswg.org/css-conditional-3/#at-supports)

### Miscellaneous

- [CSS Variables](https://www.w3.org/TR/css-variables/)


## Features unsupported

- [`@namespace`](https://www.w3.org/TR/selectors-4/#attrnmsp): not used anymore, and it's more tight
to the usage of XHTML, which is something that Rome doesn't support;


