use crate::prelude::*;
use rome_js_syntax::TsTypeAssertionAssignmentFields;

use crate::FormatNodeFields;
use rome_js_syntax::TsTypeAssertionAssignment;

impl FormatNodeFields<TsTypeAssertionAssignment> for FormatNodeRule<TsTypeAssertionAssignment> {
    fn format_fields(
        node: &TsTypeAssertionAssignment,
        formatter: &JsFormatter,
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
                formatter
                    .delimited(
                        &l_angle_token?,
                        formatted![formatter, [ty.format()]]?,
                        &r_angle_token?,
                    )
                    .soft_block_indent()
                    .finish()?,
                assignment.format()
            ]
        ]
    }
}
