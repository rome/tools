use crate::prelude::*;

use crate::builders::format_delimited;
use crate::parentheses::NeedsParentheses;
use rome_formatter::write;
use rome_js_syntax::{JsSyntaxNode, TsTupleType, TsTupleTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTupleType;

impl FormatNodeRule<TsTupleType> for FormatTsTupleType {
    fn fmt_fields(&self, node: &TsTupleType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTupleTypeFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_delimited(&l_brack_token?, &elements.format(), &r_brack_token?,)
                    .soft_block_indent()
            ]
        )
    }

    fn needs_parentheses(&self, item: &TsTupleType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsTupleType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
