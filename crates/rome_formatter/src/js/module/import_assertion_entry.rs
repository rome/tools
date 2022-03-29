use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsImportAssertionEntry;
use rome_js_syntax::JsImportAssertionEntryFields;

impl ToFormatElement for JsImportAssertionEntry {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsImportAssertionEntryFields {
            key,
            colon_token,
            value_token,
        } = self.as_fields();

        Ok(format_elements![
            key.format(formatter)?,
            colon_token.format(formatter)?,
            space_token(),
            value_token.format(formatter)?,
        ])
    }
}
