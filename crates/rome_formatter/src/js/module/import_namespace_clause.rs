use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsImportNamespaceClause;

impl ToFormatElement for JsImportNamespaceClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let star = self.star_token().format(formatter)?;
        let as_token = self.as_token().format(formatter)?;
        let local_name = self.local_name().format(formatter)?;
        let source = self.source().format(formatter)?;
        let from = self.from_token().format(formatter)?;
        let assertion = self
            .assertion()
            .format_with_or_empty(formatter, |assertion| {
                format_elements![space_token(), assertion]
            })?;
        Ok(format_elements![
            star,
            space_token(),
            as_token,
            space_token(),
            local_name,
            space_token(),
            from,
            space_token(),
            source,
            assertion
        ])
    }
}
