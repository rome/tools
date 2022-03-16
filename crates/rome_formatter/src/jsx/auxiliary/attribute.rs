use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
<<<<<<< HEAD:crates/rome_formatter/src/jsx/auxiliary/attribute.rs
use rome_js_syntax::{AstNode, JsxAttribute};
impl ToFormatElement for JsxAttribute {
=======
use rome_js_syntax::{AstNode, JsxName};
impl ToFormatElement for JsxName {
>>>>>>> 408dcd197df3a39964878660447b8154b594bbcb:crates/rome_formatter/src/jsx/auxiliary/name.rs
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_verbatim(self.syntax()))
    }
}
