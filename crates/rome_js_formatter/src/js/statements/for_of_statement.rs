use crate::prelude::*;
use crate::utils::FormatBodyStatement;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsForOfStatement;

use rome_js_syntax::JsForOfStatementFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsForOfStatement;

impl FormatNodeRule<JsForOfStatement> for FormatJsForOfStatement {
    fn fmt_fields(&self, node: &JsForOfStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsForOfStatementFields {
            for_token,
            await_token,
            l_paren_token,
            initializer,
            of_token,
            expression,
            r_paren_token,
            body,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                for_token.format(),
                space(),
                await_token
                    .format()
                    .with_or_empty(|token, f| write![f, [token, space()]]),
                l_paren_token.format(),
                group(&initializer.format()),
                space(),
                of_token.format(),
                space(),
                expression.format(),
                r_paren_token.format(),
                FormatBodyStatement::new(&body?)
            ])]
        )
    }
}
