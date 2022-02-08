use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsSequenceExpression;

impl ToFormatElement for JsSequenceExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.left().format(formatter)?,
            self.comma_token().format(formatter)?,
            space_token(),
            self.right().format(formatter)?,
        ])
    }
}
