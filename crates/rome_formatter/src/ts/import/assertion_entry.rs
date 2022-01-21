use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsImportAssertionEntry;

impl ToFormatElement for JsImportAssertionEntry {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_token(&self.key()?)?,
            formatter.format_token(&self.colon_token()?)?,
            space_token(),
            formatter.format_token(&self.value_token()?)?,
        ])
    }
}
