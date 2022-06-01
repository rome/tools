use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::JsxString;

impl FormatNodeFields<JsxString> for FormatNodeRule<JsxString> {
    fn format_fields(node: &JsxString, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        formatted![formatter, [node.value_token().format()]]
    }
}
