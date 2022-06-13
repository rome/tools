use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsNonPrimitiveType, TsNonPrimitiveTypeFields};

impl FormatNodeFields<TsNonPrimitiveType> for FormatNodeRule<TsNonPrimitiveType> {
    fn fmt_fields(node: &TsNonPrimitiveType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNonPrimitiveTypeFields { object_token } = node.as_fields();

        write![f, [object_token.format()]]
    }
}
