use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{JsExportDefaultDeclarationClause, JsExportDefaultDeclarationClauseFields};

impl FormatNodeFields<JsExportDefaultDeclarationClause>
    for FormatNodeRule<JsExportDefaultDeclarationClause>
{
    fn format_fields(
        node: &JsExportDefaultDeclarationClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsExportDefaultDeclarationClauseFields {
            default_token,
            declaration,
            semicolon_token: _,
        } = node.as_fields();

        formatted![
            formatter,
            [default_token.format(), space_token(), declaration.format()]
        ]
    }
}
