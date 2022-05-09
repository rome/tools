use crate::formatter::verbatim_node;
use crate::generated::FormatJsxChildList;
use crate::prelude::*;
use rome_js_syntax::JsxChildList;
use rome_rowan::AstNode;

impl FormatRule<JsxChildList> for FormatJsxChildList {
    fn format(node: &JsxChildList, formatter: &Formatter) -> FormatResult<FormatElement> {
        verbatim_node(node.syntax()).format(formatter)
    }
}
