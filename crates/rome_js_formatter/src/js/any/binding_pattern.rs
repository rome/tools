//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsAnyBindingPattern;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyBindingPattern;
impl FormatRule<JsAnyBindingPattern> for FormatJsAnyBindingPattern {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyBindingPattern, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyBindingPattern::JsAnyBinding(node) => node.format().fmt(f),
            JsAnyBindingPattern::JsArrayBindingPattern(node) => node.format().fmt(f),
            JsAnyBindingPattern::JsObjectBindingPattern(node) => node.format().fmt(f),
        }
    }
}
