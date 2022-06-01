use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsNumberLiteralType, TsNumberLiteralTypeFields};

impl FormatNodeFields<TsNumberLiteralType> for FormatNodeRule<TsNumberLiteralType> {
    fn format_fields(
        node: &TsNumberLiteralType,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsNumberLiteralTypeFields {
            minus_token,
            literal_token,
        } = node.as_fields();
        formatted![formatter, [minus_token.format(), literal_token.format()]]
    }
}
