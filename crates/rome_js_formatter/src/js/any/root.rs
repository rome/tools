//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyRoot;
use crate::prelude::*;
use rome_js_syntax::JsAnyRoot;
impl FormatRule<JsAnyRoot> for FormatJsAnyRoot {
    type Context = JsFormatContext;
    fn format(node: &JsAnyRoot, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        match node {
            JsAnyRoot::JsScript(node) => formatted![formatter, [node.format()]],
            JsAnyRoot::JsModule(node) => formatted![formatter, [node.format()]],
            JsAnyRoot::JsExpressionSnipped(node) => formatted![formatter, [node.format()]],
        }
    }
}
