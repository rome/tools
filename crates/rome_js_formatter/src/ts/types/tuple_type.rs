use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsTupleType, TsTupleTypeFields};

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
}
