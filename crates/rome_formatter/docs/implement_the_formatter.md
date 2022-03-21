# Implement the formatter

Our formatter is node based. Meaning that each AST node knows how to format itself. In order to implement
the formatting, a node has to implement the trait `ToFormatElement`.

Usually the developer doesn't have to manually write it, `rome` has an automatic code generation that does 
everything for us when the grammar is actually implemented. By default, all nodes will format verbatim,
meaning that the formatter will print tokens and trivia as they are.

Our formatter has its own [internal IR](https://en.wikipedia.org/wiki/Intermediate_representation), it creates its own abstraction from an AST.

The developer won't be creating directly this IR, but they will use a series of utilities that will help
to create this IR. The whole IR is represented by the `enum` `FormatElement`. Please refer to its internal
documentation to understand the meaning of each variant.


## Rules to follow when implementing a formatter

1. Use the `*Fields` struct to extract all the tokens/nodes
   ```rust
    impl ToFormatElement for JsExportDefaultExpressionClause {
        fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
            let JsExportDefaultExpressionClauseFields {
                default_token,
                expression,
                semicolon_token,
            } = self.as_fields();
        }
   }
   ```
2. When using `.as_fields()` with the destructuring, don't use the `..` feature. Prefer extracting all fields and ignore them
   using the `_`
   ```rust
   impl ToFormatElement for JsExportDefaultExpressionClause {
        fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
            let JsExportDefaultExpressionClauseFields {
                default_token,
                expression: _,
                semicolon_token
            } = self.as_fields();
        }
   }
   ```
   The reason why we want to promote this pattern is because we want to make explicit when a token/node is excluded;
3. use the APIs provided by `format_element.rs` and `formatter` and `formatter_traits.rs`. 
   1. `formatter_element.rs` exposes a series of utilities to craft the formatter IR; please refer to their internal
   documentation to understand what the utilities are for;
   2. `formatter` exposes a set of functions to help to format some recurring patterns; please refer to their internal
   documentation to understand how to use them and when;
   3. `formatter_traits.rs`: with these traits, we give the ability to nodes and tokens to implements certain methods
   that are exposed based on its type. If you have a good IDE support, this feature will help you. For example:
   ```rust
      impl ToFormatElement for JsExportDefaultExpressionClause {
        fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
            let JsExportDefaultExpressionClauseFields {
                default_token,
                expression, // it's a mandatory node
                semicolon_token, // this is not a mandatory node
            } = self.as_fields();
            let element = expression.format(formatter)?;
            let element = expression.format_with(formatter, |element| {
                format_element![element , space_token()]        
            })?;
            let semicolon = semicolon_token.format_or(formatter, || space_token())?;
            let semicolon = semicolon_token.format_or_empty(formatter)?;
            let semicolon = semicolon_token.format_with_or_empty(formatter, |semicolon_element| {
                format_element![semicolon_element, space_token()]  
            })?;
        }
   }
   ```