use crate::prelude::*;

use crate::utils::format_with_semicolon;

use rome_js_syntax::JsExportNamedFromClause;
use rome_js_syntax::JsExportNamedFromClauseFields;

impl FormatNode for JsExportNamedFromClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExportNamedFromClauseFields {
            type_token,
            l_curly_token,
            specifiers,
            r_curly_token,
            from_token,
            source,
            assertion,
            semicolon_token,
        } = self.as_fields();

        let type_token =
            type_token.with_or_empty(|token| formatted![formatter, token, space_token()]);

        let specifiers = specifiers.format(formatter)?;

        let list = formatter.format_delimited_soft_block_spaces(
            &l_curly_token?,
            specifiers,
            &r_curly_token?,
        )?;

        let from = from_token.format(formatter)?;
        let source = source.format(formatter)?;
        let assertion =
            assertion.with_or_empty(|assertion| formatted![formatter, space_token(), assertion]);

        format_with_semicolon(
            formatter,
            formatted![
                formatter,
                type_token,
                list,
                space_token(),
                from,
                space_token(),
                source,
                assertion,
            ]?,
            semicolon_token,
        )
    }
}
