use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::JsUnknown;
use rome_rowan::AstNode;

impl FormatNodeFields<JsUnknown> for FormatNodeRule<JsUnknown> {
    fn fmt_fields(node: &JsUnknown, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_unknown_node(node.syntax()).fmt(formatter)
    }
}
