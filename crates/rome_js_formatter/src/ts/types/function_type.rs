use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsFunctionType;
use rome_js_syntax::TsFunctionTypeFields;

impl FormatNodeFields<TsFunctionType> for FormatNodeRule<TsFunctionType> {
    fn format_fields(
        node: &TsFunctionType,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let TsFunctionTypeFields {
            parameters,
            fat_arrow_token,
            type_parameters,
            return_type,
        } = node.as_fields();

        formatted![
            formatter,
            [
                type_parameters.format(),
                parameters.format(),
                space_token(),
                fat_arrow_token.format(),
                space_token(),
                return_type.format()
            ]
        ]
    }
}
