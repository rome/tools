use crate::prelude::*;

use rome_js_syntax::JsxOpeningElement;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxOpeningElement;

impl FormatNodeRule<JsxOpeningElement> for FormatJsxOpeningElement {
    fn fmt_fields(node: &JsxOpeningElement, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(formatter)
    }
}
