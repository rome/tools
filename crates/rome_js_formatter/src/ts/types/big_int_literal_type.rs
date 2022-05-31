use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsBigIntLiteralType, TsBigIntLiteralTypeFields};

impl FormatNodeFields<TsBigIntLiteralType> for FormatNodeRule<TsBigIntLiteralType> {
    fn format_fields(
        node: &TsBigIntLiteralType,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsBigIntLiteralTypeFields {
            minus_token,
            literal_token,
        } = node.as_fields();

        formatted![formatter, [minus_token.format(), literal_token.format()]]
    }
}
