use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsStaticMemberAssignment;

impl ToFormatElement for JsStaticMemberAssignment {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.object().format(formatter)?,
            self.dot_token().format(formatter)?,
            self.member().format(formatter)?,
        ])
    }
}
