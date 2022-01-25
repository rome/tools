use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyNamedImport;

impl ToFormatElement for JsAnyNamedImport {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyNamedImport::JsNamedImportSpecifiers(e) => e.to_format_element(formatter),
            JsAnyNamedImport::JsNamespaceImportSpecifier(_) => todo!(),
        }
    }
}
