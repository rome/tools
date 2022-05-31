use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsAsAssignment;
use rome_js_syntax::TsAsAssignmentFields;

impl FormatNodeFields<TsAsAssignment> for FormatNodeRule<TsAsAssignment> {
    fn format_fields(
        node: &TsAsAssignment,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsAsAssignmentFields {
            assignment,
            as_token,
            ty,
        } = node.as_fields();

        formatted![
            formatter,
            [
                assignment.format(),
                space_token(),
                as_token.format(),
                space_token(),
                ty.format(),
            ]
        ]
    }
}
