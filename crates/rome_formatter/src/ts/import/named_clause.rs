use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsImportNamedClause;

impl ToFormatElement for JsImportNamedClause {
    fn to_format_element(&self, _formatter: &Formatter) -> FormatResult<FormatElement> {
        todo!()
    }
}
