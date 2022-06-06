use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsAnyType, TsAnyTypeFields};

impl FormatNodeFields<TsAnyType> for FormatNodeRule<TsAnyType> {
    fn fmt_fields(node: &TsAnyType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsAnyTypeFields { any_token } = node.as_fields();

        write![f, [any_token.format()]]
    }
}
