use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsYieldExpression;
use rome_js_syntax::JsYieldExpressionFields;

impl ToFormatElement for JsYieldExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsYieldExpressionFields {
            yield_token,
            argument,
        } = self.as_fields();

        let argument = argument.format_or_empty(formatter)?;

        Ok(format_elements![yield_token.format(formatter)?, argument])
    }
}
