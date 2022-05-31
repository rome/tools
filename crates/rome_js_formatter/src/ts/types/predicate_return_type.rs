use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsPredicateReturnType;
use rome_js_syntax::TsPredicateReturnTypeFields;

impl FormatNodeFields<TsPredicateReturnType> for FormatNodeRule<TsPredicateReturnType> {
    fn format_fields(
        node: &TsPredicateReturnType,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsPredicateReturnTypeFields {
            parameter_name,
            is_token,
            ty,
        } = node.as_fields();
        formatted![
            formatter,
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
