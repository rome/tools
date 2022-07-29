use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsNewExpressionFields;
use rome_js_syntax::{JsNewExpression, JsSyntaxKind};

#[derive(Debug, Clone, Default)]
pub struct FormatJsNewExpression;

impl FormatNodeRule<JsNewExpression> for FormatJsNewExpression {
    fn fmt_fields(&self, node: &JsNewExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsNewExpressionFields {
            new_token,
            callee,
            type_arguments,
            arguments,
        } = node.as_fields();

        write![
            f,
            [
                new_token.format(),
                space(),
                callee.format(),
                type_arguments.format(),
            ]
        ]?;

        match arguments {
            Some(arguments) => {
                write!(f, [arguments.format()])
            }
            None => {
                write!(
                    f,
                    [
                        format_inserted(JsSyntaxKind::L_PAREN),
                        format_inserted(JsSyntaxKind::R_PAREN)
                    ]
                )
            }
        }
    }
}
