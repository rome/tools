use crate::{format_elements, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsTypeAssertionExpression;
use rome_js_syntax::TsTypeAssertionExpressionFields;

impl FormatNode for TsTypeAssertionExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsTypeAssertionExpressionFields {
            l_angle_token,
            ty,
            r_angle_token,
            expression,
        } = self.as_fields();

        Ok(format_elements![
            formatter.format_delimited_soft_block_indent(
                &l_angle_token?,
                ty.format(formatter)?,
                &r_angle_token?,
            )?,
            expression.format(formatter)?
        ])
    }
}
