use crate::prelude::*;
use rome_js_syntax::TsOutModifier;
use rome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsOutModifier;
impl FormatNodeRule<TsOutModifier> for FormatTsOutModifier {
    fn fmt_fields(&self, node: &TsOutModifier, f: &mut JsFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
