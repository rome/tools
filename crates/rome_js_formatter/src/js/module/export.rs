use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsExport;
use rome_js_syntax::JsExportFields;

impl FormatNodeFields<JsExport> for FormatNodeRule<JsExport> {
    fn format_fields(node: &JsExport, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let JsExportFields {
            export_token,
            export_clause,
        } = node.as_fields();

        let export_token = export_token.format();
        let export_clause = export_clause.format();
        formatted![formatter, [export_token, space_token(), export_clause]]
    }
}
