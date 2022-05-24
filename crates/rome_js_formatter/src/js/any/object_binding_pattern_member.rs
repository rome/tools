//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyObjectBindingPatternMember;
use crate::prelude::*;
use rome_js_syntax::JsAnyObjectBindingPatternMember;
impl FormatRule<JsAnyObjectBindingPatternMember> for FormatJsAnyObjectBindingPatternMember {
    type Options = JsFormatOptions;
    fn format(
        node: &JsAnyObjectBindingPatternMember,
        formatter: &Formatter<Self::Options>,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyObjectBindingPatternMember::JsObjectBindingPatternProperty(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyObjectBindingPatternMember::JsObjectBindingPatternRest(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyObjectBindingPatternMember::JsIdentifierBinding(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyObjectBindingPatternMember::JsUnknownBinding(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
