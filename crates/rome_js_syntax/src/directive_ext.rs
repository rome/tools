use rome_rowan::{SyntaxResult, SyntaxTokenText};

use crate::{inner_text, JsDirective};

impl JsDirective {
    /// Get the inner text of a string not including the quotes.
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_factory::syntax::{JsDirective, JsSyntaxKind::*};
    /// use rome_js_factory::JsSyntaxTreeBuilder;
    /// use rome_rowan::AstNode;
    /// let mut tree_builder = JsSyntaxTreeBuilder::new();
    ///         tree_builder.start_node(JS_DIRECTIVE);
    ///         tree_builder.token(JS_STRING_LITERAL, "\"use strict\"");
    ///         tree_builder.finish_node();
    ///         let node = tree_builder.finish();
    ///         let js_directive = JsDirective::cast(node).unwrap();
    ///         let text = js_directive.inner_text().unwrap();
    ///         assert_eq!(text, "use strict")
    /// ```
    pub fn inner_text(&self) -> SyntaxResult<SyntaxTokenText> {
        Ok(inner_text(&self.value_token()?))
    }
}

#[cfg(test)]
mod tests {
    use rome_js_factory::syntax::{JsDirective, JsSyntaxKind::*};
    use rome_js_factory::JsSyntaxTreeBuilder;
    use rome_rowan::AstNode;

    #[test]
    fn js_directive_inner_text() {
        let tokens = vec!["\"use strict\"", "'use strict'"];
        for token in tokens {
            let mut tree_builder = JsSyntaxTreeBuilder::new();
            tree_builder.start_node(JS_DIRECTIVE);
            tree_builder.token(JS_STRING_LITERAL, token);
            tree_builder.finish_node();

            let node = tree_builder.finish();
            let js_directive = JsDirective::cast(node).unwrap();
            let text = js_directive.inner_text().unwrap();
            assert_eq!(text, "use strict")
        }
    }
}
