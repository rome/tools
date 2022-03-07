//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyPropertyModifier;
impl ToFormatElement for JsAnyPropertyModifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsAccessibilityModifier(node) => node.to_format_element(formatter),
            Self::JsStaticModifier(node) => node.to_format_element(formatter),
            Self::TsReadonlyModifier(node) => node.to_format_element(formatter),
            Self::TsOverrideModifier(node) => node.to_format_element(formatter),
        }
    }
}
