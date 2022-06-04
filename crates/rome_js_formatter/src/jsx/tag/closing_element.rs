use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::JsxClosingElement;
use rome_rowan::AstNode;

impl FormatNodeFields<JsxClosingElement> for FormatNodeRule<JsxClosingElement> {
    fn fmt_fields(node: &JsxClosingElement, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(formatter)
    }
}
