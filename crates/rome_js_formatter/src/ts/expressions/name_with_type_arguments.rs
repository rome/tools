use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsNameWithTypeArguments, TsNameWithTypeArgumentsFields};

impl FormatNodeFields<TsNameWithTypeArguments> for FormatNodeRule<TsNameWithTypeArguments> {
    fn format_fields(
        node: &TsNameWithTypeArguments,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsNameWithTypeArgumentsFields {
            name,
            type_arguments,
        } = node.as_fields();

        formatted![formatter, [name.format(), type_arguments.format()]]
    }
}
