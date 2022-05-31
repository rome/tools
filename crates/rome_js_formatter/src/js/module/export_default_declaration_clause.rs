use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::{JsExportDefaultDeclarationClause, JsExportDefaultDeclarationClauseFields};

impl FormatNodeFields<JsExportDefaultDeclarationClause>
    for FormatNodeRule<JsExportDefaultDeclarationClause>
{
    fn format_fields(
        node: &JsExportDefaultDeclarationClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsExportDefaultDeclarationClauseFields {
            default_token,
            declaration,
            semicolon_token: _,
        } = node.as_fields();

        write![
            f,
            [default_token.format(), space_token(), declaration.format()]
        ]
    }
}
