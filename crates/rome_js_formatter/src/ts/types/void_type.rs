use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsVoidType, TsVoidTypeFields};

impl FormatNodeFields<TsVoidType> for FormatNodeRule<TsVoidType> {
    fn fmt_fields(node: &TsVoidType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsVoidTypeFields { void_token } = node.as_fields();

        write![f, [void_token.format()]]
    }
}
