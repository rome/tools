//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyTemplateElement;
impl ToFormatElement for JsAnyTemplateElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsTemplateChunkElement(node) => node.to_format_element(formatter),
            Self::JsTemplateElement(node) => node.to_format_element(formatter),
        }
    }
}
