use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsPrivateName;
use rome_js_syntax::JsPrivateNameFields;

impl FormatNodeFields<JsPrivateName> for FormatNodeRule<JsPrivateName> {
    fn format_fields(node: &JsPrivateName, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let JsPrivateNameFields {
            hash_token,
            value_token,
        } = node.as_fields();

        formatted![formatter, [hash_token.format(), value_token.format()]]
    }
}
