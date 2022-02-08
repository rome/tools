use rslint_parser::ast::JsForVariableDeclaration;

use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

impl ToFormatElement for JsForVariableDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.kind_token().format(formatter)?,
            space_token(),
            self.declaration().format(formatter)?,
        ])
    }
}
