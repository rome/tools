use crate::{parentheses::NeedsParentheses, prelude::*};
use rome_formatter::write;
use rome_js_syntax::{TsExpressionWithTypeArguments, TsExpressionWithTypeArgumentsFields};
use rome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub struct FormatTsExpressionWithTypeArguments;
impl FormatNodeRule<TsExpressionWithTypeArguments> for FormatTsExpressionWithTypeArguments {
    fn fmt_fields(
        &self,
        node: &TsExpressionWithTypeArguments,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsExpressionWithTypeArgumentsFields {
            expression,
            arguments,
        } = node.as_fields();

        write![f, [expression.format(), arguments.format()]]
    }
}

impl NeedsParentheses for TsExpressionWithTypeArguments {
    fn needs_parentheses_with_parent(&self, _: &rome_js_syntax::JsSyntaxNode) -> bool {
        false
    }
}
