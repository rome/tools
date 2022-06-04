use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::ImportMeta;
use rome_js_syntax::ImportMetaFields;

impl FormatNodeFields<ImportMeta> for FormatNodeRule<ImportMeta> {
    fn fmt_fields(node: &ImportMeta, f: &mut JsFormatter) -> FormatResult<()> {
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
