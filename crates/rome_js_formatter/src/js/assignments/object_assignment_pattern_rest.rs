use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsObjectAssignmentPatternRest;
use rome_js_syntax::JsObjectAssignmentPatternRestFields;

impl FormatNodeFields<JsObjectAssignmentPatternRest>
    for FormatNodeRule<JsObjectAssignmentPatternRest>
{
    fn format_fields(
        node: &JsObjectAssignmentPatternRest,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsObjectAssignmentPatternRestFields {
            dotdotdot_token,
            target,
        } = node.as_fields();

        formatted![formatter, [dotdotdot_token.format(), target.format()?,]]
    }
}
