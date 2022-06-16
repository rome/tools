use crate::prelude::*;

use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};
use crate::FormatNodeFields;
use rome_js_syntax::JsExpressionStatement;
use rome_js_syntax::JsStringLiteralExpression;
use rome_js_syntax::JsStringLiteralExpressionFields;
use rome_rowan::AstNode;

impl FormatNodeFields<JsStringLiteralExpression> for FormatNodeRule<JsStringLiteralExpression> {
    fn fmt_fields(node: &JsStringLiteralExpression, f: &mut JsFormatter) -> FormatResult<()> {
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
            format_parenthesize(&value_token, &formatted, &value_token).fmt(f)
        } else {
            formatted.fmt(f)
        }
    }
}
