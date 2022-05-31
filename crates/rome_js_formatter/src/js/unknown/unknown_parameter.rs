use crate::prelude::*;

use crate::formatter::unknown_node;
use crate::FormatNodeFields;
use rome_js_syntax::JsUnknownParameter;
use rome_rowan::AstNode;

impl FormatNodeFields<JsUnknownParameter> for FormatNodeRule<JsUnknownParameter> {
    fn format_fields(node: &JsUnknownParameter, formatter: &mut JsFormatter) -> FormatResult<()> {
        unknown_node(node.syntax()).format(formatter)
    }
}
