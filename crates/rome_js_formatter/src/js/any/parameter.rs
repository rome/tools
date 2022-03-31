//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyParameter;
impl ToFormatElement for JsAnyParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyFormalParameter(node) => node.to_format_element(formatter),
            Self::JsRestParameter(node) => node.to_format_element(formatter),
            Self::TsThisParameter(node) => node.to_format_element(formatter),
        }
    }
}
