use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsAwaitExpression;
use rome_js_syntax::JsAwaitExpressionFields;

impl ToFormatElement for JsAwaitExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsAwaitExpressionFields {
            await_token,
            argument,
        } = self.as_fields();

        Ok(format_elements![
            await_token.format(formatter)?,
            space_token(),
            argument.format(formatter)?,
        ])
    }
}
