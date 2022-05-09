use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsTupleType, TsTupleTypeFields};

impl FormatNodeFields<TsTupleType> for FormatNodeRule<TsTupleType> {
    fn format_fields(node: &TsTupleType, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsTupleTypeFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = node.as_fields();

        formatter.format_delimited_soft_block_indent(
            &l_brack_token?,
            formatted![formatter, elements.format()]?,
            &r_brack_token?,
        )
    }
}
