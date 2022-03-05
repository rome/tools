use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_syntax::TsNonNullAssertionExpression;
use rslint_syntax::TsNonNullAssertionExpressionFields;

impl ToFormatElement for TsNonNullAssertionExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsNonNullAssertionExpressionFields {
            expression,
            excl_token,
        } = self.as_fields();

        Ok(format_elements![
            expression.format(formatter)?,
            excl_token.format(formatter)?
        ])
    }
}
