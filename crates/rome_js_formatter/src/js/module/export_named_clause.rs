use crate::prelude::*;

use crate::utils::format_with_semicolon;

use crate::FormatNodeFields;
use rome_js_syntax::JsExportNamedClause;
use rome_js_syntax::JsExportNamedClauseFields;

impl FormatNodeFields<JsExportNamedClause> for FormatNodeRule<JsExportNamedClause> {
    fn format_fields(
        node: &JsExportNamedClause,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        let JsExportNamedClauseFields {
            type_token,
            l_curly_token,
            specifiers,
            r_curly_token,
            semicolon_token,
        } = node.as_fields();

        let specifiers = specifiers.format();

        let list = formatter.format_delimited_soft_block_spaces(
            &l_curly_token?,
            formatted![formatter, [specifiers]]?,
            &r_curly_token?,
        )?;

        format_with_semicolon(
            formatter,
            formatted![
                formatter,
                [
                    type_token
                        .format()
                        .with_or_empty(|token| formatted![formatter, [token, space_token()]]),
                    list
                ]
            ]?,
            semicolon_token,
        )
    }
}
