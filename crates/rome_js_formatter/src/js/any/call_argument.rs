//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyCallArgument;
use crate::prelude::*;
use rome_js_syntax::JsAnyCallArgument;
impl FormatRule<JsAnyCallArgument> for FormatJsAnyCallArgument {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyCallArgument, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyCallArgument::JsAnyExpression(node) => node.format().fmt(f),
            JsAnyCallArgument::JsSpread(node) => node.format().fmt(f),
        }
    }
}
