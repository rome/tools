use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsAssignmentWithDefault;
use rome_js_syntax::JsAssignmentWithDefaultFields;

impl FormatNodeFields<JsAssignmentWithDefault> for FormatNodeRule<JsAssignmentWithDefault> {
    fn format_fields(
        node: &JsAssignmentWithDefault,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsAssignmentWithDefaultFields {
            pattern,
            eq_token,
            default,
        } = node.as_fields();

        formatted![
            formatter,
            [
                pattern.format(),
                space_token(),
                eq_token.format(),
                space_token(),
                default.format(),
            ]
        ]
    }
}
