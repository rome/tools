//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsAnyMethodSignatureModifier;
impl ToFormatElement for TsAnyMethodSignatureModifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsAccessibilityModifier(node) => node.format(formatter),
            Self::JsStaticModifier(node) => node.format(formatter),
            Self::TsOverrideModifier(node) => node.format(formatter),
            Self::TsAbstractModifier(node) => node.format(formatter),
        }
    }
}
