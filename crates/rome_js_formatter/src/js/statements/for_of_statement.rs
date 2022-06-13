use crate::prelude::*;
use crate::utils::FormatBodyStatement;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsForOfStatement;

use crate::FormatNodeFields;
use rome_js_syntax::JsForOfStatementFields;

impl FormatNodeFields<JsForOfStatement> for FormatNodeRule<JsForOfStatement> {
    fn fmt_fields(node: &JsForOfStatement, f: &mut JsFormatter) -> FormatResult<()> {
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
            [group_elements(&format_args![
                for_token.format(),
                space_token(),
                await_token
                    .format()
                    .with_or_empty(|token, f| write![f, [token, space_token()]]),
                l_paren_token.format(),
                group_elements(&initializer.format()),
                space_token(),
                of_token.format(),
                space_token(),
                expression.format(),
                r_paren_token.format(),
                FormatBodyStatement::new(&body?)
            ])]
        )
    }
}
