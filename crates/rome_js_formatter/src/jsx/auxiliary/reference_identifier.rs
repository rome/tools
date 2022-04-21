use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::JsxReferenceIdentifier;

impl FormatNode for JsxReferenceIdentifier {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.value_token().format(formatter)
    }
}
