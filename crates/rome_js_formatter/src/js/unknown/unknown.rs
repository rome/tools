use crate::formatter::unknown_node;
use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::JsUnknown;
use rome_rowan::AstNode;

impl FormatNodeFields<JsUnknown> for FormatNodeRule<JsUnknown> {
    fn format_fields(node: &JsUnknown, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        unknown_node(node.syntax()).format(formatter)
    }
}
