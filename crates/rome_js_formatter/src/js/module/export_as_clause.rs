use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsExportAsClause;
use rome_js_syntax::JsExportAsClauseFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsExportAsClause;

impl FormatNodeRule<JsExportAsClause> for FormatJsExportAsClause {
    fn fmt_fields(node: &JsExportAsClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExportAsClauseFields {
            as_token,
            exported_name,
        } = node.as_fields();

        write![
            f,
            [as_token.format(), space_token(), exported_name.format()]
        ]
    }
}
