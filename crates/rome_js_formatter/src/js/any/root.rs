//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyRoot;
use crate::prelude::*;
use rome_js_syntax::JsAnyRoot;
impl FormatRule<JsAnyRoot> for FormatJsAnyRoot {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyRoot, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyRoot::JsScript(node) => node.format().fmt(f),
            JsAnyRoot::JsModule(node) => node.format().fmt(f),
            JsAnyRoot::JsExpressionSnipped(node) => node.format().fmt(f),
        }
    }
}
