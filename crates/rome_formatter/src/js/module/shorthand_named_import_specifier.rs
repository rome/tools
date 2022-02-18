use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsShorthandNamedImportSpecifier;
use rslint_parser::ast::JsShorthandNamedImportSpecifierFields;

impl ToFormatElement for JsShorthandNamedImportSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsShorthandNamedImportSpecifierFields {
            type_token,
            local_name,
        } = self.as_fields();

        local_name.format(formatter)
    }
}
