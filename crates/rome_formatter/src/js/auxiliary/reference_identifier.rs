use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsReferenceIdentifier;

impl ToFormatElement for JsReferenceIdentifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.value_token().format(formatter)
    }
}
