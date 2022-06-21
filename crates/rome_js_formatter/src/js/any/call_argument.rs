//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyCallArgument;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyCallArgument;
impl FormatRule<JsAnyCallArgument> for FormatJsAnyCallArgument {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyCallArgument, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyCallArgument::JsAnyExpression(node) => node.format().fmt(f),
            JsAnyCallArgument::JsSpread(node) => node.format().fmt(f),
        }
    }
}
