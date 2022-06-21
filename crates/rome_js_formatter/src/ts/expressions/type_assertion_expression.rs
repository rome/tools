use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::TsTypeAssertionExpression;
use rome_js_syntax::TsTypeAssertionExpressionFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeAssertionExpression;

impl FormatNodeRule<TsTypeAssertionExpression> for FormatTsTypeAssertionExpression {
    fn fmt_fields(node: &TsTypeAssertionExpression, f: &mut JsFormatter) -> FormatResult<()> {
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
