use crate::prelude::*;
use rome_formatter::write;
use rome_js_syntax::TsTypeAssertionAssignmentFields;

use crate::FormatNodeFields;
use rome_js_syntax::TsTypeAssertionAssignment;

impl FormatNodeFields<TsTypeAssertionAssignment> for FormatNodeRule<TsTypeAssertionAssignment> {
    fn fmt_fields(node: &TsTypeAssertionAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeAssertionAssignmentFields {
            l_angle_token,
            ty,
            r_angle_token,
            assignment,
        } = node.as_fields();

        write![
            f,
            [
                format_delimited(&l_angle_token?, &ty.format(), &r_angle_token?)
                    .soft_block_indent(),
                assignment.format()
            ]
        ]
    }
}
