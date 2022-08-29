use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use rome_formatter::write;
use rome_js_syntax::{JsSyntaxNode, TsAnyType, TsAnyTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsAnyType;

impl FormatNodeRule<TsAnyType> for FormatTsAnyType {
    fn fmt_fields(&self, node: &TsAnyType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsAnyTypeFields { any_token } = node.as_fields();

        write![f, [any_token.format()]]
    }

    fn needs_parentheses(&self, item: &TsAnyType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsAnyType {
    #[inline]
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
