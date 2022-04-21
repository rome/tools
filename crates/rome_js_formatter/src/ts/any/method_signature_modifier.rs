//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::TsAnyMethodSignatureModifier;
impl Format for TsAnyMethodSignatureModifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsAccessibilityModifier(node) => node.format(formatter),
            Self::JsStaticModifier(node) => node.format(formatter),
            Self::TsOverrideModifier(node) => node.format(formatter),
            Self::TsAbstractModifier(node) => node.format(formatter),
        }
    }
}
