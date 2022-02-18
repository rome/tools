use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsDefaultImportSpecifier;
use rslint_parser::ast::JsDefaultImportSpecifierFields;

impl ToFormatElement for JsDefaultImportSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsDefaultImportSpecifierFields {
            local_name,
            trailing_comma_token,
        } = self.as_fields();

        Ok(format_elements![
            local_name.format(formatter)?,
            trailing_comma_token.format(formatter)?
        ])
    }
}
