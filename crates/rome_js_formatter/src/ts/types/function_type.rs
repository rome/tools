use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsFunctionType;
use rome_js_syntax::TsFunctionTypeFields;

impl FormatNodeFields<TsFunctionType> for FormatNodeRule<TsFunctionType> {
    fn fmt_fields(node: &TsFunctionType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsFunctionTypeFields {
            parameters,
            fat_arrow_token,
            type_parameters,
            return_type,
        } = node.as_fields();

        write![
            f,
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
