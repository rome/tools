use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::TsAsAssignment;
use rome_js_syntax::TsAsAssignmentFields;

impl ToFormatElement for TsAsAssignment {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsAsAssignmentFields {
            assignment,
            as_token,
            ty,
        } = self.as_fields();

        Ok(format_elements![
            assignment.format(formatter)?,
            space_token(),
            as_token.format(formatter)?,
            space_token(),
            ty.format(formatter)?,
        ])
    }
}
