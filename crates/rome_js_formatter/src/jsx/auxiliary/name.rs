use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{JsxName, JsxNameFields};

impl FormatNodeFields<JsxName> for FormatNodeRule<JsxName> {
    fn format_fields(node: &JsxName, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let JsxNameFields { value_token } = node.as_fields();

        formatted![formatter, [value_token.format()]]
    }
}
