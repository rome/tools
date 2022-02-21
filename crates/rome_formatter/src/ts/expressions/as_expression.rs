use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsAsExpression;
use rslint_parser::ast::TsAsExpressionFields;

impl ToFormatElement for TsAsExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsAsExpressionFields {
            ty,
            as_token,
            expression,
        } = self.as_fields();

        Ok(format_elements![
            expression.format(formatter)?,
            space_token(),
            as_token.format(formatter)?,
            space_token(),
            ty.format(formatter)?,
        ])
    }
}
