use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsUnknownStatement;
use rome_rowan::AstNode;

impl FormatNodeFields<JsUnknownStatement> for FormatNodeRule<JsUnknownStatement> {
    fn fmt_fields(node: &JsUnknownStatement, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_unknown_node(node.syntax()).fmt(formatter)
    }
}
