use crate::{format_elements, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::{token, FormatResult};

use rome_js_syntax::JsAwaitExpressionFields;
use rome_js_syntax::{JsAwaitExpression, JsSyntaxKind};
use rome_rowan::AstNode;

impl FormatNode for JsAwaitExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsAwaitExpressionFields {
            await_token,
            argument,
        } = self.as_fields();

        let needs_parenthesis = self.syntax().parent().map_or(false, |parent| {
            matches!(
                parent.kind(),
                JsSyntaxKind::JS_SPREAD
                    | JsSyntaxKind::TS_AS_EXPRESSION
                    | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION
                    | JsSyntaxKind::JS_LOGICAL_EXPRESSION
                    | JsSyntaxKind::JS_BINARY_EXPRESSION
            )
        });

        let formatted = format_elements![
            await_token.format(formatter)?,
            space_token(),
            argument.format(formatter)?,
        ];
        if needs_parenthesis {
            Ok(format_elements![token("("), formatted, token(")"),])
        } else {
            Ok(formatted)
        }
    }
}
