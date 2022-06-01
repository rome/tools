use crate::formatter::verbatim_node;
use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::JsxOpeningElement;
use rome_rowan::AstNode;

impl FormatNodeFields<JsxOpeningElement> for FormatNodeRule<JsxOpeningElement> {
    fn format_fields(
        node: &JsxOpeningElement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        verbatim_node(node.syntax()).format(formatter)
    }
}
