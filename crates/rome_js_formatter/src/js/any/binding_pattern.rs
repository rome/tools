//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyBindingPattern;
use crate::prelude::*;
use rome_js_syntax::JsAnyBindingPattern;
impl FormatRule<JsAnyBindingPattern> for FormatJsAnyBindingPattern {
    fn format(node: &JsAnyBindingPattern, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            JsAnyBindingPattern::JsAnyBinding(node) => formatted![formatter, node.format()],
            JsAnyBindingPattern::JsArrayBindingPattern(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyBindingPattern::JsObjectBindingPattern(node) => {
                formatted![formatter, node.format()]
            }
        }
    }
}
