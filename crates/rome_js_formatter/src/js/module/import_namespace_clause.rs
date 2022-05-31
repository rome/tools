use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsImportNamespaceClause;
use rome_js_syntax::JsImportNamespaceClauseFields;

impl FormatNodeFields<JsImportNamespaceClause> for FormatNodeRule<JsImportNamespaceClause> {
    fn format_fields(
        node: &JsImportNamespaceClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsImportNamespaceClauseFields {
            type_token,
            star_token,
            as_token,
            local_name,
            from_token,
            source,
            assertion,
        } = node.as_fields();

        formatted![
            formatter,
            [
                type_token
                    .format()
                    .with_or_empty(|token| formatted![formatter, [token, space_token()]]),
                star_token.format(),
                space_token(),
                as_token.format(),
                space_token(),
                local_name.format(),
                space_token(),
                from_token.format(),
                space_token(),
                source.format(),
                assertion
                    .format()
                    .with_or_empty(|assertion| formatted![formatter, [space_token(), assertion]])
            ]
        ]
    }
}
