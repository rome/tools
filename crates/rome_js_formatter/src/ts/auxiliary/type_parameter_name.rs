use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::{TsTypeParameterName, TsTypeParameterNameFields};

impl FormatNodeFields<TsTypeParameterName> for FormatNodeRule<TsTypeParameterName> {
    fn format_fields(node: &TsTypeParameterName, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeParameterNameFields { ident_token } = node.as_fields();

        write![f, [ident_token.format()]]
    }
}
