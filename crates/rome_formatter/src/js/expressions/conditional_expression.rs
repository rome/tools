use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsConditionalExpression;

impl ToFormatElement for JsConditionalExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.test().format(formatter)?,
            space_token(),
            self.question_mark_token().format(formatter)?,
            space_token(),
            self.consequent().format(formatter)?,
            space_token(),
            self.colon_token().format(formatter)?,
            space_token(),
            self.alternate().format(formatter)?,
        ])
    }
}
