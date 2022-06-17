use crate::prelude::*;
use crate::utils::node_has_leading_newline;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsObjectType, TsObjectTypeFields};

impl FormatNodeFields<TsObjectType> for FormatNodeRule<TsObjectType> {
    fn fmt_fields(node: &TsObjectType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsObjectTypeFields {
            l_curly_token,
            members,
            r_curly_token,
        } = node.as_fields();

        if node_has_leading_newline(members.syntax()) {
            write!(
                f,
                [
                    format_delimited(&l_curly_token?, &members.format(), &r_curly_token?)
                        .block_indent()
                ]
            )
        } else {
            write!(
                f,
                [
                    format_delimited(&l_curly_token?, &members.format(), &r_curly_token?,)
                        .soft_block_spaces()
                ]
            )
        }
    }
}
