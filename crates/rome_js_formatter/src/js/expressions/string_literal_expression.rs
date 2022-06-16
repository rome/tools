use crate::prelude::*;
use rome_formatter::{write, Buffer};

use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};
use crate::FormatNodeFields;
use rome_js_syntax::JsStringLiteralExpression;
use rome_js_syntax::JsStringLiteralExpressionFields;
use rome_js_syntax::{JsExpressionStatement, JsSyntaxKind};
use rome_rowan::AstNode;

impl FormatNodeFields<JsStringLiteralExpression> for FormatNodeRule<JsStringLiteralExpression> {
    fn fmt_fields(node: &JsStringLiteralExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsStringLiteralExpressionFields { value_token } = node.as_fields();

        let value_token = value_token?;
        let syntax_node = node.syntax();
        let parent = syntax_node.parent();

        let needs_parenthesis = parent.and_then(JsExpressionStatement::cast).is_some();

        if needs_parenthesis {
            format_parenthesize(
                JsSyntaxKind::L_PAREN,
                &FormatLiteralStringToken::new(&value_token, StringLiteralParentKind::Expression),
                JsSyntaxKind::R_PAREN,
            )
            .fmt(f)
        } else {
            write!(
                f,
                [FormatLiteralStringToken::new(
                    &value_token,
                    StringLiteralParentKind::Expression
                )]
            )
        }
    }
}
