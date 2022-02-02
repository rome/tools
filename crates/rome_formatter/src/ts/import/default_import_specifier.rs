use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsDefaultImportSpecifier;

impl ToFormatElement for JsDefaultImportSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.local_name().format(formatter)?,
            self.trailing_comma_token().format(formatter)?
        ])
    }
}
