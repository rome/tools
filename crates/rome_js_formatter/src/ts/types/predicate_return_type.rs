use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::TsPredicateReturnType;
use rome_js_syntax::TsPredicateReturnTypeFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsPredicateReturnType;

impl FormatNodeRule<TsPredicateReturnType> for FormatTsPredicateReturnType {
    fn fmt_fields(node: &TsPredicateReturnType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsPredicateReturnTypeFields {
            parameter_name,
            is_token,
            ty,
        } = node.as_fields();
        write![
            f,
            [
                parameter_name.format(),
                space_token(),
                is_token.format(),
                space_token(),
                ty.format()
            ]
        ]
    }
}
