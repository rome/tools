use crate::prelude::*;
use rome_js_syntax::TsConstModifier;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsConstModifier;

impl FormatNodeRule<TsConstModifier> for FormatTsConstModifier {
    fn fmt_fields(&self, node: &TsConstModifier, f: &mut JsFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
