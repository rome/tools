use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsSpread;
use rome_js_syntax::JsSpreadFields;

impl FormatNodeFields<JsSpread> for FormatNodeRule<JsSpread> {
    fn format_fields(node: &JsSpread, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let JsSpreadFields {
            dotdotdot_token,
            argument,
        } = node.as_fields();

        formatted![formatter, [dotdotdot_token.format(), argument.format()]]
    }
}
