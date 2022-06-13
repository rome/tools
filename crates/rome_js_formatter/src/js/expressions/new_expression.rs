use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsNewExpression;
use rome_js_syntax::JsNewExpressionFields;

impl FormatNodeFields<JsNewExpression> for FormatNodeRule<JsNewExpression> {
    fn fmt_fields(node: &JsNewExpression, f: &mut JsFormatter) -> FormatResult<()> {
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
                space_token(),
                callee.format(),
                type_arguments.format(),
            ]
        ]?;

        match arguments {
            Some(arguments) => {
                write!(f, [arguments.format()])
            }
            None => {
                write!(f, [token("()")])
            }
        }
    }
}
