use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsUnknownExpression;
use rome_rowan::AstNode;

impl FormatNodeFields<JsUnknownExpression> for FormatNodeRule<JsUnknownExpression> {
    fn fmt_fields(node: &JsUnknownExpression, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_unknown_node(node.syntax()).fmt(formatter)
    }
}
