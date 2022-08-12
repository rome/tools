use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use rome_formatter::write;
use rome_js_syntax::{JsSyntaxKind, TsAsExpression, TsTypeAssertionExpressionFields};
use rome_js_syntax::{JsSyntaxNode, TsTypeAssertionExpression};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeAssertionExpression;

impl FormatNodeRule<TsTypeAssertionExpression> for FormatTsTypeAssertionExpression {
    fn fmt_fields(
        &self,
        node: &TsTypeAssertionExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsTypeAssertionExpressionFields {
            l_angle_token,
            ty,
            r_angle_token,
            expression,
        } = node.as_fields();

        write![
            f,
            [
                format_delimited(&l_angle_token?, &ty.format(), &r_angle_token?,)
                    .soft_block_indent(),
                expression.format()
            ]
        ]
    }
}
