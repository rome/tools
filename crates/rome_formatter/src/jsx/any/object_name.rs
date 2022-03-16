//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsxAnyObjectName;
impl ToFormatElement for JsxAnyObjectName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsxReferenceIdentifier(node) => node.to_format_element(formatter),
            Self::JsxMemberName(node) => node.to_format_element(formatter),
            Self::JsxNamespaceName(node) => node.to_format_element(formatter),
        }
    }
}
