//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::JsAnyRoot;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyRoot;
impl FormatRule<JsAnyRoot> for FormatJsAnyRoot {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyRoot, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyRoot::JsScript(node) => node.format().fmt(f),
            JsAnyRoot::JsModule(node) => node.format().fmt(f),
            JsAnyRoot::JsExpressionSnipped(node) => node.format().fmt(f),
        }
    }
}
