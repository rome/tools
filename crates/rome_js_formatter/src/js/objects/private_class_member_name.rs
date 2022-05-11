use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsPrivateClassMemberName;
use rome_js_syntax::JsPrivateClassMemberNameFields;

impl FormatNodeFields<JsPrivateClassMemberName> for FormatNodeRule<JsPrivateClassMemberName> {
    fn format_fields(
        node: &JsPrivateClassMemberName,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        let JsPrivateClassMemberNameFields {
            hash_token,
            id_token,
        } = node.as_fields();

        formatted![formatter, hash_token.format(), id_token.format(),]
    }
}
