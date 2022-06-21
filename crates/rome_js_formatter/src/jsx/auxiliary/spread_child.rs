use crate::prelude::*;

use rome_js_syntax::JsxSpreadChild;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxSpreadChild;

impl FormatNodeRule<JsxSpreadChild> for FormatJsxSpreadChild {
    fn fmt_fields(node: &JsxSpreadChild, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(formatter)
    }
}
