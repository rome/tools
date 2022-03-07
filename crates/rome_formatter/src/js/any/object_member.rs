//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyObjectMember;
impl ToFormatElement for JsAnyObjectMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsPropertyObjectMember(node) => node.to_format_element(formatter),
            Self::JsMethodObjectMember(node) => node.to_format_element(formatter),
            Self::JsGetterObjectMember(node) => node.to_format_element(formatter),
            Self::JsSetterObjectMember(node) => node.to_format_element(formatter),
            Self::JsShorthandPropertyObjectMember(node) => node.to_format_element(formatter),
            Self::JsSpread(node) => node.to_format_element(formatter),
            Self::JsUnknownMember(node) => node.to_format_element(formatter),
        }
    }
}
