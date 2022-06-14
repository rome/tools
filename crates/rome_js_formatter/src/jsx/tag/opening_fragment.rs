use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{JsxOpeningFragment, JsxOpeningFragmentFields};

impl FormatNodeFields<JsxOpeningFragment> for FormatNodeRule<JsxOpeningFragment> {
    fn fmt_fields(node: &JsxOpeningFragment, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxOpeningFragmentFields {
            r_angle_token,
            l_angle_token,
        } = node.as_fields();

        write![f, [l_angle_token.format(), r_angle_token.format()]]
    }
}
