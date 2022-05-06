use rome_formatter::FormatResult;
use rome_js_syntax::ImportMeta;

use crate::{formatted, Format, FormatElement, FormatNode, Formatter};
use rome_js_syntax::ImportMetaFields;

impl FormatNode for ImportMeta {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let ImportMetaFields {
            import_token,
            dot_token,
            meta_token,
        } = self.as_fields();

        formatted![
            formatter,
            import_token.format(formatter)?,
            dot_token.format(formatter)?,
            meta_token.format(formatter)?,
        ]
    }
}
