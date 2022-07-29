use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{JsSwitchStatement, JsSwitchStatementFields};
use rome_rowan::AstNodeList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsSwitchStatement;

impl FormatNodeRule<JsSwitchStatement> for FormatJsSwitchStatement {
    fn fmt_fields(&self, node: &JsSwitchStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSwitchStatementFields {
            switch_token,
            l_paren_token,
            discriminant,
            r_paren_token,
            l_curly_token,
            cases,
            r_curly_token,
        } = node.as_fields();

        let format_cases = format_with(|f| {
            if cases.is_empty() {
                hard_line_break().fmt(f)?;
            } else {
                cases.format().fmt(f)?;
            }

            Ok(())
        });

        write![
            f,
            [
                switch_token.format(),
                space(),
                format_delimited(&l_paren_token?, &discriminant.format(), &r_paren_token?,)
                    .soft_block_indent(),
                space(),
                format_delimited(&l_curly_token?, &format_cases, &r_curly_token?).block_indent()
            ]
        ]
    }
}
