use crate::prelude::*;

use rome_js_syntax::JsxElement;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxElement;

impl FormatNodeRule<JsxElement> for FormatJsxElement {
    fn fmt_fields(&self, node: &JsxElement, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(formatter)
    }
}
