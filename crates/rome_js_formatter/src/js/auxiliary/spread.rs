use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsSpread;
use rome_js_syntax::JsSpreadFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsSpread;

impl FormatNodeRule<JsSpread> for FormatJsSpread {
    fn fmt_fields(node: &JsSpread, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSpreadFields {
            dotdotdot_token,
            argument,
        } = node.as_fields();

        write![f, [dotdotdot_token.format(), argument.format()]]
    }
}
