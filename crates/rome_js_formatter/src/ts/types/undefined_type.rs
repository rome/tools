use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsUndefinedType, TsUndefinedTypeFields};

impl FormatNodeFields<TsUndefinedType> for FormatNodeRule<TsUndefinedType> {
    fn fmt_fields(node: &TsUndefinedType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsUndefinedTypeFields { undefined_token } = node.as_fields();

        write![f, [undefined_token.format()]]
    }
}
