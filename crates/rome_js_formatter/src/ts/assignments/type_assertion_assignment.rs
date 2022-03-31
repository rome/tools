use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsTypeAssertionAssignmentFields;

use rome_js_syntax::TsTypeAssertionAssignment;

impl ToFormatElement for TsTypeAssertionAssignment {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsTypeAssertionAssignmentFields {
            l_angle_token,
            ty,
            r_angle_token,
            assignment,
        } = self.as_fields();

        Ok(format_elements![
            formatter.format_delimited_soft_block_indent(
                &l_angle_token?,
                ty.format(formatter)?,
                &r_angle_token?,
            )?,
            assignment.format(formatter)?
        ])
    }
}
