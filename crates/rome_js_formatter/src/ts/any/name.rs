//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsAnyName;
impl ToFormatElement for TsAnyName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsReferenceIdentifier(node) => node.to_format_element(formatter),
            Self::TsQualifiedName(node) => node.to_format_element(formatter),
        }
    }
}
