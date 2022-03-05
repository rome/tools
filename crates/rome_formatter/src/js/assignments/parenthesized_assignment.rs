use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_syntax::JsParenthesizedAssignment;
use rslint_syntax::JsParenthesizedAssignmentFields;

impl ToFormatElement for JsParenthesizedAssignment {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsParenthesizedAssignmentFields {
            l_paren_token,
            assignment,
            r_paren_token,
        } = self.as_fields();

        Ok(format_elements![
            l_paren_token.format(formatter)?,
            assignment.format(formatter)?,
            r_paren_token.format(formatter)?,
        ])
    }
}
