use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::NewTarget;

impl ToFormatElement for NewTarget {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.new_token().format(formatter)?,
            self.dot_token().format(formatter)?,
            self.target_token().format(formatter)?,
        ])
    }
}
