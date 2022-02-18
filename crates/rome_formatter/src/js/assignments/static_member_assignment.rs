use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsStaticMemberAssignment;
use rslint_parser::ast::JsStaticMemberAssignmentFields;

impl ToFormatElement for JsStaticMemberAssignment {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsStaticMemberAssignmentFields {
            object,
            dot_token,
            member,
        } = self.as_fields();

        Ok(format_elements![
            object.format(formatter)?,
            dot_token.format(formatter)?,
            member.format(formatter)?,
        ])
    }
}
