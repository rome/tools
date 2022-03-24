//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyFormalParameter;
impl ToFormatElement for JsAnyFormalParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsFormalParameter(node) => node.format(formatter),
            Self::JsUnknownParameter(node) => node.format(formatter),
        }
    }
}
