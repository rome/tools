use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsAssertsReturnType;
use rome_js_syntax::TsAssertsReturnTypeFields;

impl FormatNodeFields<TsAssertsReturnType> for FormatNodeRule<TsAssertsReturnType> {
    fn fmt_fields(node: &TsAssertsReturnType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsAssertsReturnTypeFields {
            parameter_name,
            asserts_token,
            predicate,
        } = node.as_fields();
        write![
            f,
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
