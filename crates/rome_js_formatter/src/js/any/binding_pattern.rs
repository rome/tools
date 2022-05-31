//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyBindingPattern;
use crate::prelude::*;
use rome_js_syntax::JsAnyBindingPattern;
impl FormatRule<JsAnyBindingPattern> for FormatJsAnyBindingPattern {
    type Context = JsFormatContext;
    fn format(node: &JsAnyBindingPattern, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyBindingPattern::JsAnyBinding(node) => node.format().format(f),
            JsAnyBindingPattern::JsArrayBindingPattern(node) => node.format().format(f),
            JsAnyBindingPattern::JsObjectBindingPattern(node) => node.format().format(f),
        }
    }
}
