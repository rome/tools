use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsExportAsClause;
use rome_js_syntax::JsExportAsClauseFields;

impl FormatNodeFields<JsExportAsClause> for FormatNodeRule<JsExportAsClause> {
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
