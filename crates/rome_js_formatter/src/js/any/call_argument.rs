//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyCallArgument;
use crate::prelude::*;
use rome_js_syntax::JsAnyCallArgument;
impl FormatRule<JsAnyCallArgument> for FormatJsAnyCallArgument {
    fn format(node: &JsAnyCallArgument, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            JsAnyCallArgument::JsAnyExpression(node) => formatted![formatter, node.format()],
            JsAnyCallArgument::JsSpread(node) => formatted![formatter, node.format()],
        }
    }
}
