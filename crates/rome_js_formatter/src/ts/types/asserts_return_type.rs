use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::TsAssertsReturnType;
use rome_js_syntax::TsAssertsReturnTypeFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsAssertsReturnType;

impl FormatNodeRule<TsAssertsReturnType> for FormatTsAssertsReturnType {
    fn fmt_fields(&self, node: &TsAssertsReturnType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsAssertsReturnTypeFields {
            parameter_name,
            asserts_token,
            predicate,
        } = node.as_fields();
        write![
            f,
            [
                asserts_token.format(),
                space(),
                parameter_name.format(),
                space(),
                predicate.format()
            ]
        ]
    }
}
