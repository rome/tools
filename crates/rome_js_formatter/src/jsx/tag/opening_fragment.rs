use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{JsxOpeningFragment, JsxOpeningFragmentFields};

impl FormatNodeFields<JsxOpeningFragment> for FormatNodeRule<JsxOpeningFragment> {
    fn format_fields(
        node: &JsxOpeningFragment,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsxOpeningFragmentFields {
            r_angle_token,
            l_angle_token,
        } = node.as_fields();

        formatted![formatter, [l_angle_token.format(), r_angle_token.format()]]
    }
}
