use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsThisType, TsThisTypeFields};

impl FormatNodeFields<TsThisType> for FormatNodeRule<TsThisType> {
    fn fmt_fields(node: &TsThisType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsThisTypeFields { this_token } = node.as_fields();

        write![f, [this_token.format()]]
    }
}
