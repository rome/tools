use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsParenthesizedAssignment;

impl ToFormatElement for JsParenthesizedAssignment {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_token(&self.l_paren_token()?)?,
            formatter.format_node(self.assignment()?)?,
            formatter.format_token(&self.r_paren_token()?)?,
        ])
    }
}
