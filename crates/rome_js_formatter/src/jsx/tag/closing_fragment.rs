use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{JsxClosingFragment, JsxClosingFragmentFields};

impl FormatNodeFields<JsxClosingFragment> for FormatNodeRule<JsxClosingFragment> {
    fn fmt_fields(node: &JsxClosingFragment, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxClosingFragmentFields {
            r_angle_token,
            slash_token,
            l_angle_token,
        } = node.as_fields();

        write![
            f,
            [
                l_angle_token.format(),
                slash_token.format(),
                r_angle_token.format()
            ]
        ]
    }
}
