use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsUnknownType, TsUnknownTypeFields};

impl FormatNodeFields<TsUnknownType> for FormatNodeRule<TsUnknownType> {
    fn format_fields(node: &TsUnknownType, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let TsUnknownTypeFields { unknown_token } = node.as_fields();
        formatted![formatter, [unknown_token.format()]]
    }
}
