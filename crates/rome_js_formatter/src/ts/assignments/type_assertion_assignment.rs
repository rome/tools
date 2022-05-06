use crate::{Format, FormatElement, FormatNode, Formatter, JsFormatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsTypeAssertionAssignmentFields;

use rome_js_syntax::TsTypeAssertionAssignment;

impl FormatNode for TsTypeAssertionAssignment {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsTypeAssertionAssignmentFields {
            l_angle_token,
            ty,
            r_angle_token,
            assignment,
        } = self.as_fields();

        formatted![
            formatter,
            formatter.format_delimited_soft_block_indent(
                &l_angle_token?,
                ty.format(formatter)?,
                &r_angle_token?,
            )?,
            assignment.format(formatter)?
        ]
    }
}
