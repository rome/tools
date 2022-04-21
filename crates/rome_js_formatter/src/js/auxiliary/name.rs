use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};

use rome_js_syntax::JsName;
use rome_js_syntax::JsNameFields;

impl FormatNode for JsName {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsNameFields { value_token } = self.as_fields();

        value_token.format(formatter)
    }
}
