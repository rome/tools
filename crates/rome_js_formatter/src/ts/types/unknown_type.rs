use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsUnknownType, TsUnknownTypeFields};

impl FormatNodeFields<TsUnknownType> for FormatNodeRule<TsUnknownType> {
    fn fmt_fields(node: &TsUnknownType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsUnknownTypeFields { unknown_token } = node.as_fields();

        write![f, [unknown_token.format()]]
    }
}
