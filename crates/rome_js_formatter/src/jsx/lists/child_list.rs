use crate::prelude::*;
use rome_js_syntax::JsxChildList;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxChildList;

impl FormatRule<JsxChildList> for FormatJsxChildList {
    type Context = JsFormatContext;

    fn fmt(node: &JsxChildList, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(formatter)
    }
}
