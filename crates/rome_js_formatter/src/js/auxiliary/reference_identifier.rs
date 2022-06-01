use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsReferenceIdentifier;
use rome_js_syntax::JsReferenceIdentifierFields;

impl FormatNodeFields<JsReferenceIdentifier> for FormatNodeRule<JsReferenceIdentifier> {
    fn format_fields(
        node: &JsReferenceIdentifier,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsReferenceIdentifierFields { value_token } = node.as_fields();

        formatted![formatter, [value_token.format()]]
    }
}
