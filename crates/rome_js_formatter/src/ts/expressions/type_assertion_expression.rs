use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsTypeAssertionExpression;
use rome_js_syntax::TsTypeAssertionExpressionFields;

impl FormatNodeFields<TsTypeAssertionExpression> for FormatNodeRule<TsTypeAssertionExpression> {
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
