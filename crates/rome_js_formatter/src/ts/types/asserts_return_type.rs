use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsAssertsReturnType;
use rome_js_syntax::TsAssertsReturnTypeFields;

impl FormatNodeFields<TsAssertsReturnType> for FormatNodeRule<TsAssertsReturnType> {
    fn format_fields(
        node: &TsAssertsReturnType,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsAssertsReturnTypeFields {
            parameter_name,
            asserts_token,
            predicate,
        } = node.as_fields();
        formatted![
            formatter,
            [
                asserts_token.format(),
                space_token(),
                parameter_name.format(),
                space_token(),
                predicate.format()
            ]
        ]
    }
}
