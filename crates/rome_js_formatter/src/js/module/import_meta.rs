use rome_js_syntax::ImportMeta;

use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::ImportMetaFields;

impl FormatNode for ImportMeta {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let ImportMetaFields {
            import_token,
            dot_token,
            meta_token,
        } = self.as_fields();

        Ok(format_elements![
            import_token.format(formatter)?,
            dot_token.format(formatter)?,
            meta_token.format(formatter)?,
        ])
    }
}
