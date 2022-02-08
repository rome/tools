use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::{ast::JsNamedImportSpecifierList, AstNode};
impl ToFormatElement for JsNamedImportSpecifierList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_verbatim(self.syntax()))
    }
}