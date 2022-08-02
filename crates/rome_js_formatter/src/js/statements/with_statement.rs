use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::utils::FormatStatementBody;
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
            [group(&format_args![
                with_token.format(),
                space(),
                l_paren_token.format(),
                object.format(),
                r_paren_token.format(),
                FormatStatementBody::new(&body?)
            ])]
        )
    }
}
