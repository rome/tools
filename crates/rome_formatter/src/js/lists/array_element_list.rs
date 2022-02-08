use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::{ast::JsArrayElementList, AstNode};
impl ToFormatElement for JsArrayElementList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_verbatim(self.syntax()))
    }
}