use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::JsxName;

impl FormatNode for JsxName {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.value_token().format(formatter)
    }
}
