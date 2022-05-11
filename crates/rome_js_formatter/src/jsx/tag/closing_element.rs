use crate::formatter::verbatim_node;
use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::JsxClosingElement;
use rome_rowan::AstNode;

impl FormatNodeFields<JsxClosingElement> for FormatNodeRule<JsxClosingElement> {
    fn format_fields(
        node: &JsxClosingElement,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        verbatim_node(node.syntax()).format(formatter)
    }
}
