use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsInstanceofExpression;

impl ToFormatElement for JsInstanceofExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.left().format(formatter)?,
            space_token(),
            self.instanceof_token().format(formatter)?,
            space_token(),
            self.right().format(formatter)?,
        ])
    }
}
