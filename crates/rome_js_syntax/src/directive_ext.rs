use rome_rowan::{SyntaxResult, SyntaxTokenText, TextSize};

use crate::JsDirective;

impl JsDirective {
    /// Get the inner text of a string not including the quotes
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
    ///         let text = js_directive.inner_string_text().unwrap();
    ///         assert_eq!(text, "use strict")
    /// ```
    pub fn inner_string_text(&self) -> SyntaxResult<SyntaxTokenText> {
        let value = self.value_token()?;
        let mut text = value.token_text_trimmed();

        static QUOTES: [char; 2] = ['"', '\''];

        if text.starts_with(QUOTES) {
            let range = text.range().add_start(TextSize::from(1));
            text = text.slice(range);
        }

        if text.ends_with(QUOTES) {
            let range = text.range().sub_end(TextSize::from(1));
            text = text.slice(range);
        }

        Ok(text)
    }
}

#[cfg(test)]
mod tests {
    use rome_js_factory::syntax::{JsDirective, JsSyntaxKind::*};
    use rome_js_factory::JsSyntaxTreeBuilder;
    use rome_rowan::AstNode;

    #[test]
    fn js_directive_inner_string_text() {
        let tokens = vec!["\"use strict\"", "'use strict'"];
        for token in tokens {
            let mut tree_builder = JsSyntaxTreeBuilder::new();
            tree_builder.start_node(JS_DIRECTIVE);
            tree_builder.token(JS_STRING_LITERAL, token);
            tree_builder.finish_node();

            let node = tree_builder.finish();
            let js_directive = JsDirective::cast(node).unwrap();
            let text = js_directive.inner_string_text().unwrap();
            assert_eq!(text, "use strict")
        }
    }
}
