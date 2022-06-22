use crate::prelude::*;
use rome_js_syntax::JsxChildList;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxChildList;

impl FormatRule<JsxChildList> for FormatJsxChildList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsxChildList, f: &mut JsFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
