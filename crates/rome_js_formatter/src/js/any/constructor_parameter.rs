//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, Formatter};
use rome_formatter::{FormatElement, FormatResult};
use rome_js_syntax::JsAnyConstructorParameter;
impl Format for JsAnyConstructorParameter {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyFormalParameter(node) => node.format(formatter),
            Self::JsRestParameter(node) => node.format(formatter),
            Self::TsPropertyParameter(node) => node.format(formatter),
        }
    }
}
