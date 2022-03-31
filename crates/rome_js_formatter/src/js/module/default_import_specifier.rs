use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsDefaultImportSpecifier;
use rome_js_syntax::JsDefaultImportSpecifierFields;

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
