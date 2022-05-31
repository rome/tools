use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsArrayAssignmentPatternRestElement;
use rome_js_syntax::JsArrayAssignmentPatternRestElementFields;

impl FormatNodeFields<JsArrayAssignmentPatternRestElement>
    for FormatNodeRule<JsArrayAssignmentPatternRestElement>
{
    fn format_fields(
        node: &JsArrayAssignmentPatternRestElement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsArrayAssignmentPatternRestElementFields {
            dotdotdot_token,
            pattern,
        } = node.as_fields();

        formatted![formatter, [dotdotdot_token.format(), pattern.format()]]
    }
}
