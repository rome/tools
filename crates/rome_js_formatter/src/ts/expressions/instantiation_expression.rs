use crate::{parentheses::NeedsParentheses, prelude::*};
use rome_formatter::write;
use rome_js_syntax::{TsInstantiationExpression, TsInstantiationExpressionFields};
#[derive(Debug, Clone, Default)]
pub struct FormatTsInstantiationExpression;
impl FormatNodeRule<TsInstantiationExpression> for FormatTsInstantiationExpression {
    fn fmt_fields(
        &self,
        node: &TsInstantiationExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsInstantiationExpressionFields {
            expression,
            arguments,
        } = node.as_fields();

        write![f, [expression.format(), arguments.format()]]
    }
}

impl NeedsParentheses for TsInstantiationExpression {
    fn needs_parentheses_with_parent(&self, _: &rome_js_syntax::JsSyntaxNode) -> bool {
        false
    }
}
