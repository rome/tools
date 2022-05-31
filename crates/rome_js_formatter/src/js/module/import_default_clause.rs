use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsImportDefaultClause;
use rome_js_syntax::JsImportDefaultClauseFields;

impl FormatNodeFields<JsImportDefaultClause> for FormatNodeRule<JsImportDefaultClause> {
    fn format_fields(
        node: &JsImportDefaultClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsImportDefaultClauseFields {
            type_token,
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
