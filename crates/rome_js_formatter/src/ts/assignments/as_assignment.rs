use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::TsAsAssignment;
use rome_js_syntax::TsAsAssignmentFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsAsAssignment;

impl FormatNodeRule<TsAsAssignment> for FormatTsAsAssignment {
    fn fmt_fields(&self, node: &TsAsAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        let TsAsAssignmentFields {
            assignment,
            as_token,
            ty,
        } = node.as_fields();

        write![
            f,
            [
                assignment.format(),
                space(),
                as_token.format(),
                space(),
                ty.format(),
            ]
        ]
    }
}
