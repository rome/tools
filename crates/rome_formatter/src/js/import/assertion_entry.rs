use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsImportAssertionEntry;

impl ToFormatElement for JsImportAssertionEntry {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.key().format(formatter)?,
            self.colon_token().format(formatter)?,
            space_token(),
            self.value_token().format(formatter)?,
        ])
    }
}
