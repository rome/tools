use crate::prelude::*;
use rome_js_syntax::JsxString;

impl FormatNode for JsxString {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.value_token().format(formatter)
    }
}
