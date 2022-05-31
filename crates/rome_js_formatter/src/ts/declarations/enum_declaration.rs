use crate::prelude::*;
use crate::utils::has_leading_newline;
use crate::FormatNodeFields;
use rome_js_syntax::{TsEnumDeclaration, TsEnumDeclarationFields};
use rome_rowan::AstNode;

impl FormatNodeFields<TsEnumDeclaration> for FormatNodeRule<TsEnumDeclaration> {
    fn format_fields(
        node: &TsEnumDeclaration,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsEnumDeclarationFields {
            const_token,
            enum_token,
            id,
            members,
            l_curly_token,
            r_curly_token,
        } = node.as_fields();

        let has_newline = has_leading_newline(members.syntax());
        let list = formatter
            .delimited(
                &l_curly_token?,
                join_elements(
                    if has_newline {
                        hard_line_break()
                    } else {
                        soft_line_break_or_space()
                    },
                    formatter.format_separated(&members, || token(","))?,
                ),
                &r_curly_token?,
            )
            .soft_block_spaces()
            .finish()?;

        formatted![
            formatter,
            [
                const_token.format().with_or_empty(|const_token| formatted![
                    formatter,
                    [const_token, space_token()]
                ]),
                enum_token
                    .format()
                    .with(|enum_token| formatted![formatter, [enum_token, space_token()]]),
                id.format()
                    .with(|id| formatted![formatter, [id, space_token()]]),
                list
            ]
        ]
    }
}
