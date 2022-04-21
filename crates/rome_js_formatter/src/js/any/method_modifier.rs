//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsAnyMethodModifier;
impl Format for JsAnyMethodModifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsAccessibilityModifier(node) => node.format(formatter),
            Self::JsStaticModifier(node) => node.format(formatter),
            Self::TsOverrideModifier(node) => node.format(formatter),
        }
    }
}
