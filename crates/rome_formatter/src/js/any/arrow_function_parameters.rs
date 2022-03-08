//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyArrowFunctionParameters;
impl ToFormatElement for JsAnyArrowFunctionParameters {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsParameters(node) => node.to_format_element(formatter),
            Self::JsAnyBinding(node) => node.to_format_element(formatter),
        }
    }
}
