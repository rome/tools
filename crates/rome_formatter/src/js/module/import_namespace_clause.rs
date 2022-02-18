use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsImportNamespaceClause;
use rslint_parser::ast::JsImportNamespaceClauseFields;

impl ToFormatElement for JsImportNamespaceClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsImportNamespaceClauseFields {
            type_token,
            star_token,
            as_token,
            local_name,
            from_token,
            source,
            assertion,
        } = self.as_fields();

        let star = star_token.format(formatter)?;
        let as_token = as_token.format(formatter)?;
        let local_name = local_name.format(formatter)?;
        let source = source.format(formatter)?;
        let from = from_token.format(formatter)?;
        let assertion = assertion.format_with_or_empty(formatter, |assertion| {
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
