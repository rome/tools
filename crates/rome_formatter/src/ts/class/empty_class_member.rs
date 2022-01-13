use crate::{empty_element, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsEmptyClassMember;

impl ToFormatElement for JsEmptyClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_replaced_token(&self.semicolon_token()?, empty_element())
    }
}
