//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyArrayElement;
use crate::prelude::*;
use rome_js_syntax::JsAnyArrayElement;
impl FormatRule<JsAnyArrayElement> for FormatJsAnyArrayElement {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyArrayElement, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyArrayElement::JsAnyExpression(node) => node.format().fmt(f),
            JsAnyArrayElement::JsSpread(node) => node.format().fmt(f),
            JsAnyArrayElement::JsArrayHole(node) => node.format().fmt(f),
        }
    }
}
