use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsNeverType, TsNeverTypeFields};

impl FormatNodeFields<TsNeverType> for FormatNodeRule<TsNeverType> {
    fn format_fields(
        node: &TsNeverType,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let TsNeverTypeFields { never_token } = node.as_fields();
        formatted![formatter, [never_token.format()]]
    }
}
