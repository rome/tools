use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsNameWithTypeArguments, TsNameWithTypeArgumentsFields};

impl FormatNodeFields<TsNameWithTypeArguments> for FormatNodeRule<TsNameWithTypeArguments> {
    fn fmt_fields(node: &TsNameWithTypeArguments, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNameWithTypeArgumentsFields {
            name,
            type_arguments,
        } = node.as_fields();

        write![f, [name.format(), type_arguments.format()]]
    }
}
