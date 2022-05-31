use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsTupleType, TsTupleTypeFields};

impl FormatNodeFields<TsTupleType> for FormatNodeRule<TsTupleType> {
    fn format_fields(node: &TsTupleType, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let TsTupleTypeFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = node.as_fields();

        formatter
            .delimited(
                &l_brack_token?,
                formatted![formatter, [elements.format()]]?,
                &r_brack_token?,
            )
            .soft_block_indent()
            .finish()
    }
}
