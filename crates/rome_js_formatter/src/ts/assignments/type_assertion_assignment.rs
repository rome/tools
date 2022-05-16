use crate::prelude::*;
use rome_js_syntax::TsTypeAssertionAssignmentFields;

use crate::FormatNodeFields;
use rome_js_syntax::TsTypeAssertionAssignment;

impl FormatNodeFields<TsTypeAssertionAssignment> for FormatNodeRule<TsTypeAssertionAssignment> {
    fn format_fields(
        node: &TsTypeAssertionAssignment,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let TsTypeAssertionAssignmentFields {
            l_angle_token,
            ty,
            r_angle_token,
            assignment,
        } = node.as_fields();

        formatted![
            formatter,
            [
                formatter.format_delimited_soft_block_indent(
                    &l_angle_token?,
                    formatted![formatter, [ty.format()]]?,
                    &r_angle_token?,
                )?,
                assignment.format()
            ]
        ]
    }
}
