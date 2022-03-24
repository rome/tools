//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyObjectMember;
impl ToFormatElement for JsAnyObjectMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsPropertyObjectMember(node) => node.format(formatter),
            Self::JsMethodObjectMember(node) => node.format(formatter),
            Self::JsGetterObjectMember(node) => node.format(formatter),
            Self::JsSetterObjectMember(node) => node.format(formatter),
            Self::JsShorthandPropertyObjectMember(node) => node.format(formatter),
            Self::JsSpread(node) => node.format(formatter),
            Self::JsUnknownMember(node) => node.format(formatter),
        }
    }
}
