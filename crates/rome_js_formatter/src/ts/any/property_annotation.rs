//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsAnyPropertyAnnotation;
impl ToFormatElement for TsAnyPropertyAnnotation {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsTypeAnnotation(node) => node.to_format_element(formatter),
            Self::TsOptionalPropertyAnnotation(node) => node.to_format_element(formatter),
            Self::TsDefinitePropertyAnnotation(node) => node.to_format_element(formatter),
        }
    }
}
