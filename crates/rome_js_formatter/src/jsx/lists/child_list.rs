use crate::formatter::verbatim_node;
use crate::generated::FormatJsxChildList;
use crate::prelude::*;
use rome_js_syntax::JsxChildList;
use rome_rowan::AstNode;

impl FormatRule<JsxChildList> for FormatJsxChildList {
    type Context = JsFormatContext;

    fn format(node: &JsxChildList, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        verbatim_node(node.syntax()).format(formatter)
    }
}
