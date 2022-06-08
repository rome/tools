//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyBindingPattern;
use crate::prelude::*;
use rome_js_syntax::JsAnyBindingPattern;
impl FormatRule<JsAnyBindingPattern> for FormatJsAnyBindingPattern {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyBindingPattern, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyBindingPattern::JsAnyBinding(node) => node.format().fmt(f),
            JsAnyBindingPattern::JsArrayBindingPattern(node) => node.format().fmt(f),
            JsAnyBindingPattern::JsObjectBindingPattern(node) => node.format().fmt(f),
        }
    }
}
