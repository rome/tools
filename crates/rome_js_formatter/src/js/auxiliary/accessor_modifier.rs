use crate::prelude::*;
use rome_js_syntax::JsAccessorModifier;
use rome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAccessorModifier;
impl FormatNodeRule<JsAccessorModifier> for FormatJsAccessorModifier {
    fn fmt_fields(&self, node: &JsAccessorModifier, f: &mut JsFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
