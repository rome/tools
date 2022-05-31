use crate::prelude::*;
use crate::utils::FormatInterpreterToken;
use rome_formatter::write;

use crate::formatter::FormatNodeExtension;
use crate::FormatNodeFields;
use rome_js_syntax::JsScript;
use rome_js_syntax::JsScriptFields;

impl FormatNodeFields<JsScript> for FormatNodeRule<JsScript> {
    fn format_fields(node: &JsScript, f: &mut JsFormatter) -> FormatResult<()> {
        let JsScriptFields {
            interpreter_token,
            directives,
            statements,
            eof_token,
        } = node.as_fields();

        write![
            f,
            [
                FormatInterpreterToken::new(interpreter_token.as_ref()),
                directives.format(),
            ]
        ]?;

        let mut join = f.join_nodes_with_hardline();

        for stmt in statements {
            join.entry(stmt.syntax(), &stmt.format_or_verbatim());
        }

        join.finish()?;

        write![
            f,
            [
                f.format_replaced(&eof_token?, &empty_element()),
                hard_line_break()
            ]
        ]
    }
}
