use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsUnknownBinding;
use rome_rowan::AstNode;

impl FormatNodeFields<JsUnknownBinding> for FormatNodeRule<JsUnknownBinding> {
    fn fmt_fields(node: &JsUnknownBinding, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_unknown_node(node.syntax()).fmt(formatter)
    }
}
