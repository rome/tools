use crate::generated::FormatJsxChildList;
use crate::prelude::*;
use rome_js_syntax::JsxChildList;
use rome_rowan::AstNode;

impl FormatRule<JsxChildList> for FormatJsxChildList {
    type Context = JsFormatContext;

    fn fmt(node: &JsxChildList, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(formatter)
    }
}
