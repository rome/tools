use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsLogicalExpression;

impl ToFormatElement for JsLogicalExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.left().format(formatter)?,
            space_token(),
            self.operator().format(formatter)?,
            space_token(),
            self.right().format(formatter)?,
        ])
    }
}
