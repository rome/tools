use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsxReferenceIdentifier;

impl FormatNodeFields<JsxReferenceIdentifier> for FormatNodeRule<JsxReferenceIdentifier> {
    fn fmt_fields(node: &JsxReferenceIdentifier, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [node.value_token().format()]]
    }
}
