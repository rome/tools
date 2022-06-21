use crate::prelude::*;
use crate::utils::FormatBodyStatement;

use rome_formatter::{format_args, write};
use rome_js_syntax::JsForInStatement;
use rome_js_syntax::JsForInStatementFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsForInStatement;

impl FormatNodeRule<JsForInStatement> for FormatJsForInStatement {
    fn fmt_fields(&self, node: &JsForInStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsForInStatementFields {
            for_token,
            l_paren_token,
            initializer,
            in_token,
            expression,
            r_paren_token,
            body,
        } = node.as_fields();

        let for_token = for_token.format();
        let initializer = initializer.format();
        let in_token = in_token.format();
        let expression = expression.format();

        write!(
            f,
            [group_elements(&format_args!(
                for_token,
                space_token(),
                l_paren_token.format(),
                group_elements(&initializer),
                space_token(),
                in_token,
                space_token(),
                expression,
                r_paren_token.format(),
                FormatBodyStatement::new(&body?)
            ))]
        )
    }
}
