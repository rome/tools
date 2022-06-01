use crate::prelude::*;
use rome_js_syntax::ImportMeta;

use crate::FormatNodeFields;
use rome_js_syntax::ImportMetaFields;

impl FormatNodeFields<ImportMeta> for FormatNodeRule<ImportMeta> {
    fn format_fields(node: &ImportMeta, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let ImportMetaFields {
            import_token,
            dot_token,
            meta_token,
        } = node.as_fields();

        formatted![
            formatter,
            [
                import_token.format(),
                dot_token.format(),
                meta_token.format(),
            ]
        ]
    }
}
