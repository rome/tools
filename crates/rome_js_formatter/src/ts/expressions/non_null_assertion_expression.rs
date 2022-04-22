use crate::{format_elements, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsNonNullAssertionExpression;
use rome_js_syntax::TsNonNullAssertionExpressionFields;

impl FormatNode for TsNonNullAssertionExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
