use crate::prelude::*;

use rome_js_syntax::JsUnknownBinding;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsUnknownBinding;

impl FormatNodeRule<JsUnknownBinding> for FormatJsUnknownBinding {
    fn fmt_fields(&self, node: &JsUnknownBinding, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_unknown_node(node.syntax()).fmt(formatter)
    }
}
