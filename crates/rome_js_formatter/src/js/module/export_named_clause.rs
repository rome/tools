use crate::prelude::*;

use crate::utils::format_with_semicolon;

use crate::FormatNodeFields;
use rome_js_syntax::JsExportNamedClause;
use rome_js_syntax::JsExportNamedClauseFields;

impl FormatNodeFields<JsExportNamedClause> for FormatNodeRule<JsExportNamedClause> {
    fn format_fields(
        node: &JsExportNamedClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsExportNamedClauseFields {
            type_token,
            l_curly_token,
            specifiers,
            r_curly_token,
            semicolon_token,
        } = node.as_fields();

        let specifiers = specifiers.format();

        let list = formatter
            .delimited(
                &l_curly_token?,
                formatted![formatter, [specifiers]]?,
                &r_curly_token?,
            )
            .soft_block_spaces()
            .finish()?;

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
