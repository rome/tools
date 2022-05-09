use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsTypeAssertionExpression;
use rome_js_syntax::TsTypeAssertionExpressionFields;

impl FormatNodeFields<TsTypeAssertionExpression> for FormatNodeRule<TsTypeAssertionExpression> {
    fn format_fields(
        node: &TsTypeAssertionExpression,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        let TsTypeAssertionExpressionFields {
            l_angle_token,
            ty,
            r_angle_token,
            expression,
        } = node.as_fields();

        formatted![
            formatter,
            formatter.format_delimited_soft_block_indent(
                &l_angle_token?,
                formatted![formatter, ty.format()]?,
                &r_angle_token?,
            )?,
            expression.format()
        ]
    }
}
