use crate::{empty_element, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsEmptyStatement;

impl ToFormatElement for JsEmptyStatement {
    fn to_format_element(&self, _formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(empty_element())
    }
}
