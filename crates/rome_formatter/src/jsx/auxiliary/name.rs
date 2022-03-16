use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::{AstNode, JsxName};
impl ToFormatElement for JsxName {
use rome_js_syntax::{AstNode, JsxAttribute};
impl ToFormatElement for JsxAttribute {
use rome_js_syntax::{AstNode, JsxName};
impl ToFormatElement for JsxName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_verbatim(self.syntax()))
    }
}
