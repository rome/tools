use crate::prelude::*;

use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};

use rome_js_syntax::JsExpressionStatement;
use rome_js_syntax::JsStringLiteralExpression;
use rome_js_syntax::JsStringLiteralExpressionFields;
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

        let needs_parenthesis = node
            .syntax()
            .parent()
            .and_then(JsExpressionStatement::cast)
            .is_some();

        if needs_parenthesis {
            format_parenthesize(
                Some(value_token.clone()),
                &formatted,
                Some(value_token.clone()),
            )
            .fmt(f)
        } else {
            formatted.fmt(f)
        }
    }
}
