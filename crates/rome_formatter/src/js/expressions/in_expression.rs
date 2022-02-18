use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsInExpression;
use rslint_parser::ast::JsInExpressionFields;

impl ToFormatElement for JsInExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsInExpressionFields {
            property,
            in_token,
            object,
        } = self.as_fields();

        Ok(format_elements![
            property.format(formatter)?,
            space_token(),
            in_token.format(formatter)?,
            space_token(),
            object.format(formatter)?,
        ])
    }
}
