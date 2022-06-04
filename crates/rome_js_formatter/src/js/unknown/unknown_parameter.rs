use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsUnknownParameter;
use rome_rowan::AstNode;

impl FormatNodeFields<JsUnknownParameter> for FormatNodeRule<JsUnknownParameter> {
    fn fmt_fields(node: &JsUnknownParameter, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_unknown_node(node.syntax()).fmt(formatter)
    }
}
