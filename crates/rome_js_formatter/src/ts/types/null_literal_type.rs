use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsNullLiteralType, TsNullLiteralTypeFields};

impl FormatNodeFields<TsNullLiteralType> for FormatNodeRule<TsNullLiteralType> {
    fn format_fields(
        node: &TsNullLiteralType,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsNullLiteralTypeFields { literal_token } = node.as_fields();
        formatted![formatter, [literal_token.format()]]
    }
}
