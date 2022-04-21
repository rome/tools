//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, Formatter};
use rome_formatter::{FormatElement, FormatResult};
use rome_js_syntax::JsAnyObjectBindingPatternMember;
impl Format for JsAnyObjectBindingPatternMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsObjectBindingPatternProperty(node) => node.format(formatter),
            Self::JsObjectBindingPatternRest(node) => node.format(formatter),
            Self::JsObjectBindingPatternShorthandProperty(node) => node.format(formatter),
            Self::JsIdentifierBinding(node) => node.format(formatter),
            Self::JsUnknownBinding(node) => node.format(formatter),
        }
    }
}
