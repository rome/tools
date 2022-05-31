use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::FormatNodeFields;
use rome_js_syntax::JsExport;
use rome_js_syntax::JsExportFields;

impl FormatNodeFields<JsExport> for FormatNodeRule<JsExport> {
    fn format_fields(node: &JsExport, f: &mut JsFormatter) -> FormatResult<()> {
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
