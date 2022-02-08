use rslint_parser::ast::ImportMeta;

use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

impl ToFormatElement for ImportMeta {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.import_token().format(formatter)?,
            self.dot_token().format(formatter)?,
            self.meta_token().format(formatter)?,
        ])
    }
}
