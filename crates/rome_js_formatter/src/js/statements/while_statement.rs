use crate::prelude::*;

use crate::utils::FormatBodyStatement;

use rome_formatter::write;
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
            [
                while_token.format(),
                space(),
                format_delimited(&l_paren_token?, &test.format(), &r_paren_token?,)
                    .soft_block_indent(),
                FormatBodyStatement::new(&body?)
            ]
        )
    }
}
