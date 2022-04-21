use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};
use rome_js_syntax::TsAsExpression;
use rome_js_syntax::TsAsExpressionFields;

impl FormatNode for TsAsExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
