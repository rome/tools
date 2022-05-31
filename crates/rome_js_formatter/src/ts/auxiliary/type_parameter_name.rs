use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsTypeParameterName, TsTypeParameterNameFields};

impl FormatNodeFields<TsTypeParameterName> for FormatNodeRule<TsTypeParameterName> {
    fn format_fields(
        node: &TsTypeParameterName,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsTypeParameterNameFields { ident_token } = node.as_fields();

        formatted![formatter, [ident_token.format()]]
    }
}
