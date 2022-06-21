use crate::prelude::*;
use crate::utils::node_has_leading_newline;

use rome_formatter::write;
use rome_js_syntax::{TsObjectType, TsObjectTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsObjectType;

impl FormatNodeRule<TsObjectType> for FormatTsObjectType {
    fn fmt_fields(&self, node: &TsObjectType, f: &mut JsFormatter) -> FormatResult<()> {
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
