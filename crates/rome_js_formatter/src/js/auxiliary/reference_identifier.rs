use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsReferenceIdentifier;
use rome_js_syntax::JsReferenceIdentifierFields;

impl FormatNodeFields<JsReferenceIdentifier> for FormatNodeRule<JsReferenceIdentifier> {
    fn fmt_fields(node: &JsReferenceIdentifier, f: &mut JsFormatter) -> FormatResult<()> {
        let JsReferenceIdentifierFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }
}
