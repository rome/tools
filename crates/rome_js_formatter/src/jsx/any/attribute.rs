//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, Formatter};
use rome_formatter::{FormatElement, FormatResult};
use rome_js_syntax::JsxAnyAttribute;
impl Format for JsxAnyAttribute {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsxAttribute(node) => node.format(formatter),
            Self::JsxSpreadAttribute(node) => node.format(formatter),
        }
    }
}
