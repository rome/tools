//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsAnyPropertySignatureModifier;
impl ToFormatElement for TsAnyPropertySignatureModifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
