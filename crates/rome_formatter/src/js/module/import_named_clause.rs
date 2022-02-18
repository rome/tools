use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsImportNamedClause;
use rslint_parser::ast::JsImportNamedClauseFields;

impl ToFormatElement for JsImportNamedClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsImportNamedClauseFields {
            type_token,
            default_specifier,
            named_import,
            from_token,
            source,
            assertion,
        } = self.as_fields();

        let source = source.format(formatter)?;

        let default = default_specifier.format_with_or_empty(formatter, |specifier| {
            format_elements![specifier, space_token()]
        })?;
        let from = from_token.format(formatter)?;
        let name = named_import.format(formatter)?;
        let assertion = assertion.format_with_or_empty(formatter, |assertion| {
            format_elements![space_token(), assertion]
        })?;
        Ok(format_elements![
            default,
            name,
            space_token(),
            from,
            space_token(),
            source,
            assertion
        ])
    }
}
