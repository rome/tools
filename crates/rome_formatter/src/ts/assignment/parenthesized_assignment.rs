use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsParenthesizedAssignment;

impl ToFormatElement for JsParenthesizedAssignment {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.l_paren_token().format(formatter)?,
            self.assignment().format(formatter)?,
            self.r_paren_token().format(formatter)?,
        ])
    }
}
