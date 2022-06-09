use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsAsAssignment;
use rome_js_syntax::TsAsAssignmentFields;

impl FormatNodeFields<TsAsAssignment> for FormatNodeRule<TsAsAssignment> {
    fn fmt_fields(node: &TsAsAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        let TsAsAssignmentFields {
            assignment,
            as_token,
            ty,
        } = node.as_fields();

        write![
            f,
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
