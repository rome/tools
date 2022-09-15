use crate::prelude::*;

use crate::utils::FormatStatementBody;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsWhileStatement;
use rome_js_syntax::JsWhileStatementFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsWhileStatement;

impl FormatNodeRule<JsWhileStatement> for FormatJsWhileStatement {
    fn fmt_fields(&self, node: &JsWhileStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsWhileStatementFields {
            while_token,
            l_paren_token,
            test,
            r_paren_token,
            body,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                while_token.format(),
                space(),
                l_paren_token.format(),
                group(&soft_block_indent(&test.format())),
                r_paren_token.format(),
                FormatStatementBody::new(&body?)
            ])]
        )
    }
}
