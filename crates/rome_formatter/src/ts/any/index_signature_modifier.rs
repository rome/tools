//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsAnyIndexSignatureModifier;
impl ToFormatElement for TsAnyIndexSignatureModifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsStaticModifier(node) => node.to_format_element(formatter),
            Self::TsReadonlyModifier(node) => node.to_format_element(formatter),
        }
    }
}
