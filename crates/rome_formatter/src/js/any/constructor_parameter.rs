//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyConstructorParameter;
impl ToFormatElement for JsAnyConstructorParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyFormalParameter(node) => node.format(formatter),
            Self::JsRestParameter(node) => node.format(formatter),
            Self::TsPropertyParameter(node) => node.format(formatter),
        }
    }
}
