use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsShorthandNamedImportSpecifier;

impl ToFormatElement for JsShorthandNamedImportSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_node(self.local_name()?)
    }
}
