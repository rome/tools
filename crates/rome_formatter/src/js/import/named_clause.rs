use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsImportNamedClause;

impl ToFormatElement for JsImportNamedClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let source = self.source().format(formatter)?;

        let default = self
            .default_specifier()
            .format_with_or_empty(formatter, |specifier| {
                format_elements![specifier, space_token()]
            })?;
        let from = self.from_token().format(formatter)?;
        let name = self.named_import().format(formatter)?;
        let assertion = self
            .assertion()
            .format_with_or_empty(formatter, |assertion| {
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
