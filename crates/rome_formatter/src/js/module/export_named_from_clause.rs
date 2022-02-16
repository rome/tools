use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsExportNamedFromClause;

impl ToFormatElement for JsExportNamedFromClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let specifiers = self.specifiers().format(formatter)?;

        let list = formatter.format_delimited_soft_block_spaces(
            &self.l_curly_token()?,
            specifiers,
            &self.r_curly_token()?,
        )?;

        let from = self.from_token().format(formatter)?;
        let source = self.source().format(formatter)?;
        let assertion = self
            .assertion()
            .format_with_or_empty(formatter, |assertion| {
                format_elements![space_token(), assertion]
            })?;
        let semicolon = self.semicolon_token().format_or(formatter, || token(";"))?;

        Ok(format_elements![
            list,
            space_token(),
            from,
            space_token(),
            source,
            space_token(),
            assertion,
            semicolon
        ])
    }
}
