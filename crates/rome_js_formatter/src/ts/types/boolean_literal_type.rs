use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsBooleanLiteralType, TsBooleanLiteralTypeFields};

impl FormatNodeFields<TsBooleanLiteralType> for FormatNodeRule<TsBooleanLiteralType> {
    fn format_fields(
        node: &TsBooleanLiteralType,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsBooleanLiteralTypeFields { literal } = node.as_fields();
        formatted![formatter, [literal.format()]]
    }
}
