use crate::prelude::*;
use crate::utils::has_leading_newline;
use crate::FormatNodeFields;
use rome_js_syntax::{TsObjectType, TsObjectTypeFields};

impl FormatNodeFields<TsObjectType> for FormatNodeRule<TsObjectType> {
    fn format_fields(node: &TsObjectType, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let TsObjectTypeFields {
            l_curly_token,
            members,
            r_curly_token,
        } = node.as_fields();

        if has_leading_newline(members.syntax()) {
            formatter
                .delimited(
                    &l_curly_token?,
                    formatted![formatter, [members.format()]]?,
                    &r_curly_token?,
                )
                .block_indent()
                .finish()
        } else {
            formatter
                .delimited(
                    &l_curly_token?,
                    formatted![formatter, [members.format()]]?,
                    &r_curly_token?,
                )
                .soft_block_spaces()
                .finish()
        }
    }
}
