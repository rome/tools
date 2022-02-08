use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsInExpression;

impl ToFormatElement for JsInExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.property().format(formatter)?,
            space_token(),
            self.in_token().format(formatter)?,
            space_token(),
            self.object().format(formatter)?,
        ])
    }
}
