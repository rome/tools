use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsImportDefaultClause;

impl ToFormatElement for JsImportDefaultClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let local_name = self.local_name().format(formatter)?;
        let from = self.from_token().format(formatter)?;
        let source = self.source().format(formatter)?;
        let assertion = self
            .assertion()
            .format_with_or_empty(formatter, |assertion| {
                format_elements![space_token(), assertion]
            })?;

        Ok(format_elements![
            local_name,
            space_token(),
            from,
            space_token(),
            source,
            assertion
        ])
    }
}
