use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsBigintType, TsBigintTypeFields};

impl FormatNodeFields<TsBigintType> for FormatNodeRule<TsBigintType> {
    fn fmt_fields(node: &TsBigintType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsBigintTypeFields { bigint_token } = node.as_fields();

        write![f, [bigint_token.format()]]
    }
}
