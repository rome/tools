//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, Formatter};
use rome_formatter::{FormatElement, FormatResult};
use rome_js_syntax::JsAnyBinding;
impl Format for JsAnyBinding {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsIdentifierBinding(node) => node.format(formatter),
            Self::JsUnknownBinding(node) => node.format(formatter),
        }
    }
}
