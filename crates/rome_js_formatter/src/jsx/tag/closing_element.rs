use crate::prelude::*;

use rome_js_syntax::JsxClosingElement;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxClosingElement;

impl FormatNodeRule<JsxClosingElement> for FormatJsxClosingElement {
    fn fmt_fields(node: &JsxClosingElement, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(formatter)
    }
}
