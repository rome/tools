use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsSpread;
use rome_js_syntax::JsSpreadFields;

impl FormatNodeFields<JsSpread> for FormatNodeRule<JsSpread> {
    fn format_fields(node: &JsSpread, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSpreadFields {
            dotdotdot_token,
            argument,
        } = node.as_fields();

        write![f, [dotdotdot_token.format(), argument.format()]]
    }
}
