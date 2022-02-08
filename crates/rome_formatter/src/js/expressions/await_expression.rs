use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsAwaitExpression;

impl ToFormatElement for JsAwaitExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.await_token().format(formatter)?,
            space_token(),
            self.argument().format(formatter)?,
        ])
    }
}
