use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::TsConstructorType;
use rome_js_syntax::TsConstructorTypeFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsConstructorType;

impl FormatNodeRule<TsConstructorType> for FormatTsConstructorType {
    fn fmt_fields(&self, node: &TsConstructorType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsConstructorTypeFields {
            abstract_token,
            new_token,
            type_parameters,
            parameters,
            fat_arrow_token,
            return_type,
        } = node.as_fields();

        if let Some(abstract_token) = abstract_token {
            write!(f, [abstract_token.format(), space_token()])?;
        }

        write![
            f,
            [
                new_token.format(),
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
