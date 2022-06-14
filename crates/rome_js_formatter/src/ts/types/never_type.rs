use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsNeverType, TsNeverTypeFields};

impl FormatNodeFields<TsNeverType> for FormatNodeRule<TsNeverType> {
    fn fmt_fields(node: &TsNeverType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNeverTypeFields { never_token } = node.as_fields();
        write![f, [never_token.format()]]
    }
}
