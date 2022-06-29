use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsxReferenceIdentifier;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxReferenceIdentifier;

impl FormatNodeRule<JsxReferenceIdentifier> for FormatJsxReferenceIdentifier {
    fn fmt_fields(&self, node: &JsxReferenceIdentifier, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [node.value_token().format()]]
    }
}
