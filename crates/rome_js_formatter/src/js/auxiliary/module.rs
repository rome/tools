use crate::prelude::*;
use rome_formatter::write;

use crate::utils::FormatInterpreterToken;
use crate::FormatNodeFields;
use rome_js_syntax::JsModule;
use rome_js_syntax::JsModuleFields;

impl FormatNodeFields<JsModule> for FormatNodeRule<JsModule> {
    fn fmt_fields(node: &JsModule, f: &mut JsFormatter) -> FormatResult<()> {
        let JsModuleFields {
            interpreter_token,
            directives,
            items,
            eof_token,
        } = node.as_fields();

        write![
            f,
            [
                FormatInterpreterToken::new(interpreter_token.as_ref()),
                directives.format()
            ]
        ]?;

        let mut join = f.join_nodes_with_hardline();

        for node in items.iter() {
            join.entry(node.syntax(), &format_or_verbatim(&node));
        }

        join.finish()?;

        write!(
            f,
            [
                format_replaced(&eof_token?, &empty_element()),
                hard_line_break()
            ]
        )
    }
}
