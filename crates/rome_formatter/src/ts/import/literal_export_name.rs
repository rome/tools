use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsLiteralExportName;

impl ToFormatElement for JsLiteralExportName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_token(&self.value()?)
    }
}
