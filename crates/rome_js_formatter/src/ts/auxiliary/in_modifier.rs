use crate::prelude::*;
use rome_js_syntax::TsInModifier;
use rome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsInModifier;
impl FormatNodeRule<TsInModifier> for FormatTsInModifier {
    fn fmt_fields(&self, node: &TsInModifier, f: &mut JsFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
