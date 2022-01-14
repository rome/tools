use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsShorthandPropertyObjectMember;

impl ToFormatElement for JsShorthandPropertyObjectMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_node(self.name()?)
    }
}
