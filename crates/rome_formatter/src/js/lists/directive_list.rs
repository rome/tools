use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsDirectiveList;
impl ToFormatElement for JsDirectiveList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(self.clone()))
    }
}