use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsEnumDeclaration, TsEnumDeclarationFields};

impl FormatNodeFields<TsEnumDeclaration> for FormatNodeRule<TsEnumDeclaration> {
    fn format_fields(
        node: &TsEnumDeclaration,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let TsEnumDeclarationFields {
            const_token,
            enum_token,
            id,
            members,
            l_curly_token,
            r_curly_token,
        } = node.as_fields();

        let list = formatter
            .delimited(
                &l_curly_token?,
                join_elements(
                    soft_line_break_or_space(),
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
