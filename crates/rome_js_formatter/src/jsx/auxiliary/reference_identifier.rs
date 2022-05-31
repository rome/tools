use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsxReferenceIdentifier;

impl FormatNodeFields<JsxReferenceIdentifier> for FormatNodeRule<JsxReferenceIdentifier> {
    fn format_fields(node: &JsxReferenceIdentifier, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [node.value_token().format()]]
    }
}
