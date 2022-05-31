use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsImportBareClause;
use rome_js_syntax::JsImportBareClauseFields;

impl FormatNodeFields<JsImportBareClause> for FormatNodeRule<JsImportBareClause> {
    fn format_fields(
        node: &JsImportBareClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsImportBareClauseFields { source, assertion } = node.as_fields();

        formatted![
            formatter,
            [
                source.format(),
                assertion
                    .format()
                    .with_or_empty(|assertion| formatted![formatter, [space_token(), assertion]])
            ]
        ]
    }
}
