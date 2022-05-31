use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::JsxReferenceIdentifier;

impl FormatNodeFields<JsxReferenceIdentifier> for FormatNodeRule<JsxReferenceIdentifier> {
    fn format_fields(
        node: &JsxReferenceIdentifier,
        formatter: &Formatter<JsFormatContext>,
    ) -> FormatResult<FormatElement> {
        formatted![formatter, [node.value_token().format()]]
    }
}
