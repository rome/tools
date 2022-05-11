//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::TsAnyPropertySignatureModifier;
impl Format for TsAnyPropertySignatureModifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsDeclareModifier(node) => node.format(formatter),
            Self::TsAccessibilityModifier(node) => node.format(formatter),
            Self::JsStaticModifier(node) => node.format(formatter),
            Self::TsReadonlyModifier(node) => node.format(formatter),
            Self::TsOverrideModifier(node) => node.format(formatter),
            Self::TsAbstractModifier(node) => node.format(formatter),
        }
    }
}
