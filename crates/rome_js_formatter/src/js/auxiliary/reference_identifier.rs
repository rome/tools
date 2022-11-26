use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsReferenceIdentifier;
use rome_js_syntax::JsReferenceIdentifierFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsReferenceIdentifier;

impl FormatNodeRule<JsReferenceIdentifier> for FormatJsReferenceIdentifier {
    fn fmt_fields(&self, node: &JsReferenceIdentifier, f: &mut JsFormatter) -> FormatResult<()> {
        let JsReferenceIdentifierFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }
}
