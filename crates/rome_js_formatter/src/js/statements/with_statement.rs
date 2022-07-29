use crate::prelude::*;
use rome_formatter::write;

use crate::utils::FormatBodyStatement;

use rome_js_syntax::JsWithStatement;
use rome_js_syntax::JsWithStatementFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsWithStatement;

impl FormatNodeRule<JsWithStatement> for FormatJsWithStatement {
    fn fmt_fields(&self, node: &JsWithStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsWithStatementFields {
            with_token,
            l_paren_token,
            object,
            r_paren_token,
            body,
        } = node.as_fields();

        write!(
            f,
            [
                with_token.format(),
                space(),
                format_delimited(&l_paren_token?, &object.format(), &r_paren_token?,)
                    .soft_block_indent(),
                FormatBodyStatement::new(&body?)
            ]
        )
    }
}
