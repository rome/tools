//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyObjectBindingPatternMember;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyObjectBindingPatternMember;
impl FormatRule<JsAnyObjectBindingPatternMember> for FormatJsAnyObjectBindingPatternMember {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyObjectBindingPatternMember, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyObjectBindingPatternMember::JsObjectBindingPatternProperty(node) => {
                node.format().fmt(f)
            }
            JsAnyObjectBindingPatternMember::JsObjectBindingPatternRest(node) => {
                node.format().fmt(f)
            }
            JsAnyObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(node) => {
                node.format().fmt(f)
            }
            JsAnyObjectBindingPatternMember::JsUnknownBinding(node) => node.format().fmt(f),
        }
    }
}
