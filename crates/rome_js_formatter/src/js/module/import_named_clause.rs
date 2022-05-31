use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsImportNamedClause;
use rome_js_syntax::JsImportNamedClauseFields;

impl FormatNodeFields<JsImportNamedClause> for FormatNodeRule<JsImportNamedClause> {
    fn format_fields(
        node: &JsImportNamedClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsImportNamedClauseFields {
            type_token,
            default_specifier,
            named_import,
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
                default_specifier
                    .format()
                    .with_or_empty(|specifier| formatted![formatter, [specifier, space_token()]]),
                named_import.format(),
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
