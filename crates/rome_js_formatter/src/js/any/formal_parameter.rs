//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsAnyFormalParameter;
impl Format for JsAnyFormalParameter {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsFormalParameter(node) => node.format(formatter),
            Self::JsUnknownParameter(node) => node.format(formatter),
        }
    }
}
