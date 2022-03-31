use crate::{empty_element, FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsArrayHole;

impl ToFormatElement for JsArrayHole {
    fn to_format_element(&self, _: &Formatter) -> FormatResult<FormatElement> {
        Ok(empty_element())
    }
}
