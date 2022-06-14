use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsBooleanType, TsBooleanTypeFields};

impl FormatNodeFields<TsBooleanType> for FormatNodeRule<TsBooleanType> {
    fn fmt_fields(node: &TsBooleanType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsBooleanTypeFields { boolean_token } = node.as_fields();

        write![f, [boolean_token.format()]]
    }
}
