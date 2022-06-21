use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsExport;
use rome_js_syntax::JsExportFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsExport;

impl FormatNodeRule<JsExport> for FormatJsExport {
    fn fmt_fields(node: &JsExport, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExportFields {
            export_token,
            export_clause,
        } = node.as_fields();

        write![
            f,
            [export_token.format(), space_token(), export_clause.format()]
        ]
    }
}
