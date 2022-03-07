//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyFormalParameter;
impl ToFormatElement for JsAnyFormalParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsFormalParameter(node) => node.to_format_element(formatter),
            Self::JsUnknownParameter(node) => node.to_format_element(formatter),
        }
    }
}
