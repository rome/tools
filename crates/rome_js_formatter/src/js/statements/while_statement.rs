use crate::prelude::*;

use crate::utils::FormatBodyStatement;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsWhileStatement;
use rome_js_syntax::JsWhileStatementFields;

impl FormatNodeFields<JsWhileStatement> for FormatNodeRule<JsWhileStatement> {
    fn fmt_fields(node: &JsWhileStatement, f: &mut JsFormatter) -> FormatResult<()> {
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
                space_token(),
                format_delimited(&l_paren_token?, &test.format(), &r_paren_token?,)
                    .soft_block_indent(),
                FormatBodyStatement::new(&body?)
            ]
        )
    }
}
