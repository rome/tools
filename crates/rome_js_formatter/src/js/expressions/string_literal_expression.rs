use crate::utils::WrappingElement;
use crate::{FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::JsExpressionStatement;
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
            if let Some(expression_statement) = parent.and_then(JsExpressionStatement::cast) {
                let great_parent_kind = expression_statement.syntax().parent().map(|p| p.kind());
                match great_parent_kind {
                    Some(JsSyntaxKind::JS_BLOCK_STATEMENT | JsSyntaxKind::JS_MODULE_ITEM_LIST) => {
                        WrappingElement::Parenthesis
                    }
                    _ => WrappingElement::None,
                }
            } else {
                WrappingElement::None
            };
        Ok(format_string_literal_token(
            value_token,
            formatter,
            needs_parenthesis,
        ))
    }
}
