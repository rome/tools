use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsShorthandNamedImportSpecifier;

impl ToFormatElement for JsShorthandNamedImportSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.local_name().format(formatter)
    }
}
