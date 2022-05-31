use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsNonNullAssertionAssignment;
use rome_js_syntax::TsNonNullAssertionAssignmentFields;

impl FormatNodeFields<TsNonNullAssertionAssignment>
    for FormatNodeRule<TsNonNullAssertionAssignment>
{
    fn format_fields(
        node: &TsNonNullAssertionAssignment,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsNonNullAssertionAssignmentFields {
            assignment,
            excl_token,
        } = node.as_fields();
        formatted![formatter, [assignment.format(), excl_token.format()]]
    }
}
