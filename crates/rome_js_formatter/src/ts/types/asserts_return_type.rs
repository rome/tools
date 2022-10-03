use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use rome_formatter::write;
use rome_js_syntax::TsAssertsReturnTypeFields;
use rome_js_syntax::{JsSyntaxNode, TsAssertsReturnType};

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

    fn needs_parentheses(&self, item: &TsAssertsReturnType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsAssertsReturnType {
    #[inline]
    fn needs_parentheses_with_parent(&self, _: &JsSyntaxNode) -> bool {
        false
    }
}
