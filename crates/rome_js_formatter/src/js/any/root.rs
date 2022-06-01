//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyRoot;
use crate::prelude::*;
use rome_js_syntax::JsAnyRoot;
impl FormatRule<JsAnyRoot> for FormatJsAnyRoot {
    type Context = JsFormatContext;
    fn format(node: &JsAnyRoot, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyRoot::JsScript(node) => node.format().format(f),
            JsAnyRoot::JsModule(node) => node.format().format(f),
            JsAnyRoot::JsExpressionSnipped(node) => node.format().format(f),
        }
    }
}
