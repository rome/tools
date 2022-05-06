use crate::{FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::JsSyntaxKind;
use rome_rowan::AstNode;

use crate::utils::format_string_literal_token;
use rome_js_syntax::JsStringLiteralExpression;
use rome_js_syntax::JsStringLiteralExpressionFields;

impl FormatNode for JsStringLiteralExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsStringLiteralExpressionFields { value_token } = self.as_fields();

        let value_token = value_token?;
        let syntax_node = self.syntax();
        let parent = syntax_node.parent();

        let needs_parenthesis =
            if let Some(JsSyntaxKind::JS_EXPRESSION_STATEMENT) = parent.clone().map(|p| p.kind()) {
                // SAFETY: since parent's `kind` is `JsSyntaxKind::JS_EXPRESSION_STATEMENT`, it is not empty.
                let parent_parent = parent.unwrap().parent();
                matches!(
                    parent_parent.map(|p| p.kind()),
                    Some(JsSyntaxKind::JS_BLOCK_STATEMENT | JsSyntaxKind::JS_MODULE_ITEM_LIST)
                )
            // false
            } else {
                false
            };
        // value_token.text().chars().position(|ch| )
        Ok(format_string_literal_token(
            value_token,
            formatter,
            needs_parenthesis,
        ))
    }
}
