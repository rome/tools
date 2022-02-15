use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{format_elements, token, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsExportNamedClause;

impl ToFormatElement for JsExportNamedClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let specifiers = self.specifiers().format(formatter)?;

        let list = formatter.format_delimited_soft_block_spaces(
            &self.l_curly_token()?,
            specifiers,
            &self.r_curly_token()?,
        )?;

        let semicolon = self.semicolon_token().format_or(formatter, || token(";"))?;

        Ok(format_elements![list, semicolon])
    }
}
