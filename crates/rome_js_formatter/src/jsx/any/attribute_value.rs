//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, Formatter};
use rome_formatter::{FormatElement, FormatResult};
use rome_js_syntax::JsxAnyAttributeValue;
impl Format for JsxAnyAttributeValue {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsxAnyTag(node) => node.format(formatter),
            Self::JsxString(node) => node.format(formatter),
            Self::JsxExpressionAttributeValue(node) => node.format(formatter),
        }
    }
}
