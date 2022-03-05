use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_syntax::TsNonNullAssertionAssignment;
use rslint_syntax::TsNonNullAssertionAssignmentFields;

impl ToFormatElement for TsNonNullAssertionAssignment {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsNonNullAssertionAssignmentFields {
            assignment,
            excl_token,
        } = self.as_fields();
        Ok(format_elements![
            assignment.format(formatter)?,
            excl_token.format(formatter)?
        ])
    }
}
