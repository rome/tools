use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsExportFromClause;

impl ToFormatElement for JsExportFromClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let star = self.star_token().format(formatter)?;

        let export_as = self
            .export_as()
            .format_with_or_empty(formatter, |as_token| {
                format_elements![as_token, space_token()]
            })?;
        let from = self.from_token().format(formatter)?;
        let source = self.source().format(formatter)?;
        let assertion = self
            .assertion()
            .format_with_or_empty(formatter, |assertion| {
                format_elements![space_token(), assertion]
            })?;
        let semicolon = self.semicolon_token().format_or(formatter, || token(";"))?;

        Ok(format_elements![
            star,
            space_token(),
            export_as,
            from,
            space_token(),
            source,
            assertion,
            semicolon
        ])
    }
}
