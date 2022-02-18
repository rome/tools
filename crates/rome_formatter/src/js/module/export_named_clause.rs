use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{format_elements, token, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsExportNamedClause;
use rslint_parser::ast::JsExportNamedClauseFields;

impl ToFormatElement for JsExportNamedClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExportNamedClauseFields {
            type_token,
            l_curly_token,
            specifiers,
            r_curly_token,
            semicolon_token,
        } = self.as_fields();

        let specifiers = specifiers.format(formatter)?;

        let list = formatter.format_delimited_soft_block_spaces(
            &l_curly_token?,
            specifiers,
            &r_curly_token?,
        )?;

        let semicolon = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(format_elements![list, semicolon])
    }
}
