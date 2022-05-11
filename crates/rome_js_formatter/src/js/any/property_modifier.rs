//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyPropertyModifier;
impl Format for JsAnyPropertyModifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsAccessibilityModifier(node) => node.format(formatter),
            Self::JsStaticModifier(node) => node.format(formatter),
            Self::TsReadonlyModifier(node) => node.format(formatter),
            Self::TsOverrideModifier(node) => node.format(formatter),
        }
    }
}
