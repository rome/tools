use crate::prelude::*;

use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};

use rome_js_syntax::JsStringLiteralExpression;
use rome_js_syntax::JsStringLiteralExpressionFields;
use rome_js_syntax::{JsExpressionStatement, JsSyntaxKind};
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsStringLiteralExpression;

impl FormatNodeRule<JsStringLiteralExpression> for FormatJsStringLiteralExpression {
    fn fmt_fields(
        &self,
        node: &JsStringLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsStringLiteralExpressionFields { value_token } = node.as_fields();

        let value_token = value_token?;

        let formatted =
            FormatLiteralStringToken::new(&value_token, StringLiteralParentKind::Expression);

        // Prevents that a string literal expression becomes a directive
        let needs_parens =
            if let Some(expression_statement) = node.parent::<JsExpressionStatement>() {
                expression_statement
                    .syntax()
                    .parent()
                    .map_or(false, |grand_parent| {
                        matches!(
                            grand_parent.kind(),
                            JsSyntaxKind::JS_STATEMENT_LIST | JsSyntaxKind::JS_MODULE_ITEM_LIST
                        )
                    })
            } else {
                false
            };

        if needs_parens {
            format_parenthesize(Some(&value_token), &formatted, Some(&value_token)).fmt(f)
        } else {
            formatted.fmt(f)
        }
    }
}
