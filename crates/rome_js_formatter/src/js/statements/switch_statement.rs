use crate::prelude::*;
use rome_js_syntax::{JsSwitchStatement, JsSwitchStatementFields};
use rome_rowan::{AstNode, AstNodeList};

impl FormatNode for JsSwitchStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsSwitchStatementFields {
            switch_token,
            l_paren_token,
            discriminant,
            r_paren_token,
            l_curly_token,
            cases,
            r_curly_token,
        } = self.as_fields();

        Ok(hard_group_elements(formatted![
            formatter,
            switch_token.format(formatter)?,
            space_token(),
            formatter.format_delimited_soft_block_indent(
                &l_paren_token?,
                discriminant.format(formatter)?,
                &r_paren_token?,
            )?,
            space_token(),
            formatter.format_delimited_block_indent(
                &l_curly_token?,
                if cases.is_empty() {
                    hard_line_break()
                } else {
                    join_elements_hard_line(
                        cases
                            .iter()
                            .map(|node| node.syntax().clone())
                            .zip(formatter.format_all(cases)?),
                    )
                },
                &r_curly_token?
            )?
        ]?))
    }
}
