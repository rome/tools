use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsPrivateName;
use rome_js_syntax::JsPrivateNameFields;

impl FormatNodeFields<JsPrivateName> for FormatNodeRule<JsPrivateName> {
    fn format_fields(node: &JsPrivateName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsPrivateNameFields {
            hash_token,
            value_token,
        } = node.as_fields();

        write![f, [hash_token.format(), value_token.format()]]
    }
}
