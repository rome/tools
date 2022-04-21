//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, Formatter};
use rome_formatter::{FormatElement, FormatResult};
use rome_js_syntax::JsxAnyTag;
impl Format for JsxAnyTag {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsxElement(node) => node.format(formatter),
            Self::JsxSelfClosingElement(node) => node.format(formatter),
            Self::JsxFragment(node) => node.format(formatter),
        }
    }
}
