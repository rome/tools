//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::TsAnyIndexSignatureModifier;
impl Format for TsAnyIndexSignatureModifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsStaticModifier(node) => node.format(formatter),
            Self::TsReadonlyModifier(node) => node.format(formatter),
        }
    }
}
