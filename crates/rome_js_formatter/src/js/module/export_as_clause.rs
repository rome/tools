use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsExportAsClause;
use rome_js_syntax::JsExportAsClauseFields;

impl FormatNodeFields<JsExportAsClause> for FormatNodeRule<JsExportAsClause> {
    fn format_fields(
        node: &JsExportAsClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsExportAsClauseFields {
            as_token,
            exported_name,
        } = node.as_fields();

        let as_token = as_token.format();
        let exported_name = exported_name.format();

        formatted![formatter, [as_token, space_token(), exported_name]]
    }
}
