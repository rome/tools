use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::ImportMeta;
use rome_js_syntax::ImportMetaFields;

#[derive(Debug, Clone, Default)]
pub struct FormatImportMeta;

#[derive(Debug, Clone, Default)]
pub struct FormatJsImportMeta;

impl FormatNodeRule<ImportMeta> for FormatImportMeta {
    fn fmt_fields(&self, node: &ImportMeta, f: &mut JsFormatter) -> FormatResult<()> {
        let ImportMetaFields {
            import_token,
            dot_token,
            meta_token,
        } = node.as_fields();

        write![
            f,
            [
                import_token.format(),
                dot_token.format(),
                meta_token.format(),
            ]
        ]
    }
}
