use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsDefaultImportSpecifier;

impl ToFormatElement for JsDefaultImportSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_node(&self.local_name()?)?,
            formatter.format_token(&self.trailing_comma_token()?)?
        ])
    }
}
