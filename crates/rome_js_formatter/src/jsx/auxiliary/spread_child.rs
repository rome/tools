use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::JsxSpreadChild;
use rome_rowan::AstNode;

impl FormatNodeFields<JsxSpreadChild> for FormatNodeRule<JsxSpreadChild> {
    fn format_fields(node: &JsxSpreadChild, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).format(formatter)
    }
}
