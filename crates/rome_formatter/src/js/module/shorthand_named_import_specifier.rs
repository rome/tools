use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsShorthandNamedImportSpecifier;
use rslint_parser::ast::JsShorthandNamedImportSpecifierFields;

impl ToFormatElement for JsShorthandNamedImportSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsShorthandNamedImportSpecifierFields {
            type_token,
            local_name,
        } = self.as_fields();

        let type_token = type_token
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;

        let local_name = local_name.format(formatter)?;

        Ok(format_elements![type_token, local_name])
    }
}
