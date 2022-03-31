use rome_js_syntax::ImportMeta;

use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::ImportMetaFields;

impl ToFormatElement for ImportMeta {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
