use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsNumberType, TsNumberTypeFields};

impl FormatNodeFields<TsNumberType> for FormatNodeRule<TsNumberType> {
    fn fmt_fields(node: &TsNumberType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNumberTypeFields { number_token } = node.as_fields();

        write![f, [number_token.format()]]
    }
}
