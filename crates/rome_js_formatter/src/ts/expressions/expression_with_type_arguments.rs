use crate::prelude::*;
use rome_js_syntax::TsExpressionWithTypeArguments;
use rome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub struct FormatTsExpressionWithTypeArguments;
impl FormatNodeRule<TsExpressionWithTypeArguments> for FormatTsExpressionWithTypeArguments {
    fn fmt_fields(
        &self,
        node: &TsExpressionWithTypeArguments,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
