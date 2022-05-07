use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::{JsxName, JsxNameFields};

impl FormatNode for JsxName {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxNameFields { value_token } = self.as_fields();

        value_token.format(formatter)
    }
}
