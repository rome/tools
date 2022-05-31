use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{JsxClosingFragment, JsxClosingFragmentFields};

impl FormatNodeFields<JsxClosingFragment> for FormatNodeRule<JsxClosingFragment> {
    fn format_fields(
        node: &JsxClosingFragment,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsxClosingFragmentFields {
            r_angle_token,
            slash_token,
            l_angle_token,
        } = node.as_fields();

        formatted![
            formatter,
            [
                l_angle_token.format(),
                slash_token.format(),
                r_angle_token.format()
            ]
        ]
    }
}
