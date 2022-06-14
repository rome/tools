use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsTupleType, TsTupleTypeFields};

impl FormatNodeFields<TsTupleType> for FormatNodeRule<TsTupleType> {
    fn fmt_fields(node: &TsTupleType, f: &mut JsFormatter) -> FormatResult<()> {
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
