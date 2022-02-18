use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsTypeAssertionExpression;
use rslint_parser::ast::TsTypeAssertionExpressionFields;

impl ToFormatElement for TsTypeAssertionExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsTypeAssertionExpressionFields {
            l_angle_token,
            ty,
            r_angle_token,
            expression,
        } = self.as_fields();

        Ok(format_elements![
            l_angle_token.format(formatter)?,
            ty.format(formatter)?,
            r_angle_token.format(formatter)?,
            expression.format(formatter)?
        ])
    }
}
