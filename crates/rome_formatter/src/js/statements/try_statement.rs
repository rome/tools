use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsTryStatement;

impl ToFormatElement for JsTryStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.try_token().format(formatter)?,
            space_token(),
            self.body().format(formatter)?,
            space_token(),
            self.catch_clause().format(formatter)?,
        ])
    }
}
