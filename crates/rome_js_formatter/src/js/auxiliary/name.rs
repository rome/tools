use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsName;
use rome_js_syntax::JsNameFields;

impl FormatNodeFields<JsName> for FormatNodeRule<JsName> {
    fn format_fields(node: &JsName, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let JsNameFields { value_token } = node.as_fields();

        formatted![formatter, [value_token.format()]]
    }
}
