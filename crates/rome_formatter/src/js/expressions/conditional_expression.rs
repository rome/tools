use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsConditionalExpression;
use rslint_parser::ast::JsConditionalExpressionFields;

impl ToFormatElement for JsConditionalExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsConditionalExpressionFields {
            test,
            question_mark_token,
            consequent,
            colon_token,
            alternate,
        } = self.as_fields();

        Ok(format_elements![
            test.format(formatter)?,
            space_token(),
            question_mark_token.format(formatter)?,
            space_token(),
            consequent.format(formatter)?,
            space_token(),
            colon_token.format(formatter)?,
            space_token(),
            alternate.format(formatter)?,
        ])
    }
}
