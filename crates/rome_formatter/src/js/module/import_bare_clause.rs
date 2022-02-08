use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsImportBareClause;

impl ToFormatElement for JsImportBareClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let source = self.source().format(formatter)?;
        let assertion = self
            .assertion()
            .format_with_or_empty(formatter, |assertion| {
                format_elements![space_token(), assertion]
            })?;

        Ok(format_elements![source, assertion])
    }
}
