use crate::prelude::*;

use crate::formatter::unknown_node;
use crate::FormatNodeFields;
use rome_js_syntax::JsUnknownStatement;
use rome_rowan::AstNode;

impl FormatNodeFields<JsUnknownStatement> for FormatNodeRule<JsUnknownStatement> {
    fn format_fields(
        node: &JsUnknownStatement,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        unknown_node(node.syntax()).format(formatter)
    }
}
