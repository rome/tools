use crate::prelude::*;

use crate::utils::format_with_semicolon;

use crate::FormatNodeFields;
use rome_js_syntax::JsExportNamedFromClause;
use rome_js_syntax::JsExportNamedFromClauseFields;

impl FormatNodeFields<JsExportNamedFromClause> for FormatNodeRule<JsExportNamedFromClause> {
    fn format_fields(
        node: &JsExportNamedFromClause,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsExportNamedFromClauseFields {
            type_token,
            l_curly_token,
            specifiers,
            r_curly_token,
            from_token,
            source,
            assertion,
            semicolon_token,
        } = node.as_fields();

        let list = formatter
            .delimited(
                &l_curly_token?,
                formatted![formatter, [specifiers.format()]]?,
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
                    list,
                    space_token(),
                    from_token.format(),
                    space_token(),
                    source.format(),
                    assertion.format().with_or_empty(|assertion| formatted![
                        formatter,
                        [space_token(), assertion]
                    ]),
                ]
            ]?,
            semicolon_token,
        )
    }
}
