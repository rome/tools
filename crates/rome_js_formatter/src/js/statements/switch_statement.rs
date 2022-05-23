use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{JsSwitchStatement, JsSwitchStatementFields};
use rome_rowan::{AstNode, AstNodeList};

impl FormatNodeFields<JsSwitchStatement> for FormatNodeRule<JsSwitchStatement> {
    fn format_fields(
        node: &JsSwitchStatement,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsSwitchStatementFields {
            switch_token,
            l_paren_token,
            discriminant,
            r_paren_token,
            l_curly_token,
            cases,
            r_curly_token,
        } = node.as_fields();

        formatted![
            formatter,
            [
                switch_token.format(),
                space_token(),
                formatter
                    .delimited(
                        &l_paren_token?,
                        formatted![formatter, [discriminant.format()]]?,
                        &r_paren_token?,
                    )
                    .soft_block_indent()
                    .finish()?,
                space_token(),
                formatter
                    .delimited(
                        &l_curly_token?,
                        if cases.is_empty() {
                            hard_line_break()
                        } else {
                            join_elements_hard_line(
                                cases
                                    .iter()
                                    .map(|node| node.syntax().clone())
                                    .zip(formatter.format_all(cases.iter().formatted())?),
                            )
                        },
                        &r_curly_token?
                    )
                    .block_indent()
                    .finish()?
            ]
        ]
    }
}
