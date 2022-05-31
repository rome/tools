use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsInterfaceDeclaration, TsInterfaceDeclarationFields};

impl FormatNodeFields<TsInterfaceDeclaration> for FormatNodeRule<TsInterfaceDeclaration> {
    fn format_fields(
        node: &TsInterfaceDeclaration,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsInterfaceDeclarationFields {
            interface_token,
            id,
            type_parameters,
            extends_clause,
            members,
            l_curly_token,
            r_curly_token,
        } = node.as_fields();

        let members = formatter
            .delimited(
                &l_curly_token?,
                formatted![formatter, [members.format()]]?,
                &r_curly_token?,
            )
            .block_indent()
            .finish()?;
        formatted![
            formatter,
            [
                interface_token.format(),
                space_token(),
                id.format(),
                type_parameters.format(),
                space_token(),
                extends_clause
                    .format()
                    .with_or_empty(|extends| formatted![formatter, [extends, space_token()]]),
                members
            ]
        ]
    }
}
