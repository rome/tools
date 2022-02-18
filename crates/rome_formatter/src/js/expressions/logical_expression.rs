use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsLogicalExpression;
use rslint_parser::ast::JsLogicalExpressionFields;

impl ToFormatElement for JsLogicalExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsLogicalExpressionFields {
            left,
            operator,
            right,
        } = self.as_fields();

        Ok(format_elements![
            left.format(formatter)?,
            space_token(),
            operator.format(formatter)?,
            space_token(),
            right.format(formatter)?,
        ])
    }
}
