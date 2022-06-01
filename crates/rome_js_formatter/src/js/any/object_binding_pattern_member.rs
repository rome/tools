//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyObjectBindingPatternMember;
use crate::prelude::*;
use rome_js_syntax::JsAnyObjectBindingPatternMember;
impl FormatRule<JsAnyObjectBindingPatternMember> for FormatJsAnyObjectBindingPatternMember {
    type Context = JsFormatContext;
    fn format(node: &JsAnyObjectBindingPatternMember, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyObjectBindingPatternMember::JsObjectBindingPatternProperty(node) => {
                node.format().format(f)
            }
            JsAnyObjectBindingPatternMember::JsObjectBindingPatternRest(node) => {
                node.format().format(f)
            }
            JsAnyObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(node) => {
                node.format().format(f)
            }
            JsAnyObjectBindingPatternMember::JsIdentifierBinding(node) => node.format().format(f),
            JsAnyObjectBindingPatternMember::JsUnknownBinding(node) => node.format().format(f),
        }
    }
}
