use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsStringType, TsStringTypeFields};

impl FormatNodeFields<TsStringType> for FormatNodeRule<TsStringType> {
    fn fmt_fields(node: &TsStringType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsStringTypeFields { string_token } = node.as_fields();

        write![f, [string_token.format()]]
    }
}
