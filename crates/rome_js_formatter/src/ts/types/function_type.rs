use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::TsFunctionType;
use rome_js_syntax::TsFunctionTypeFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsFunctionType;

impl FormatNodeRule<TsFunctionType> for FormatTsFunctionType {
    fn fmt_fields(&self, node: &TsFunctionType, f: &mut JsFormatter) -> FormatResult<()> {
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
                space(),
                fat_arrow_token.format(),
                space(),
                return_type.format()
            ]
        ]
    }
}
