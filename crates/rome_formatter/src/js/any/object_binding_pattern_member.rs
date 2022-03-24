//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyObjectBindingPatternMember;
impl ToFormatElement for JsAnyObjectBindingPatternMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsObjectBindingPatternProperty(node) => node.format(formatter),
            Self::JsObjectBindingPatternRest(node) => node.format(formatter),
            Self::JsObjectBindingPatternShorthandProperty(node) => node.format(formatter),
            Self::JsIdentifierBinding(node) => node.format(formatter),
            Self::JsUnknownBinding(node) => node.format(formatter),
        }
    }
}
