use crate::formatter::verbatim_node;
use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::JsxElement;
use rome_rowan::AstNode;

impl FormatNodeFields<JsxElement> for FormatNodeRule<JsxElement> {
    fn format_fields(
        node: &JsxElement,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        verbatim_node(node.syntax()).format(formatter)
    }
}
