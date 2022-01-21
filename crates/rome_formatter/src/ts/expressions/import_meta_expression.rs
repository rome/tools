use rslint_parser::ast::ImportMeta;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

impl ToFormatElement for ImportMeta {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_token(&self.import_token()?)?,
            formatter.format_token(&self.dot_token()?)?,
            formatter.format_token(&self.meta_token()?)?
        ])
    }
}
