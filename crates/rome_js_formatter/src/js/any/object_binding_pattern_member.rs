//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyObjectBindingPatternMember;
impl ToFormatElement for JsAnyObjectBindingPatternMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsObjectBindingPatternProperty(node) => node.to_format_element(formatter),
            Self::JsObjectBindingPatternRest(node) => node.to_format_element(formatter),
            Self::JsObjectBindingPatternShorthandProperty(node) => {
                node.to_format_element(formatter)
            }
            Self::JsIdentifierBinding(node) => node.to_format_element(formatter),
            Self::JsUnknownBinding(node) => node.to_format_element(formatter),
        }
    }
}
