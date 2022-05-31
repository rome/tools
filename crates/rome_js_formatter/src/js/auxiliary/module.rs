use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::formatter::TryFormatNodeListExtension;
use crate::utils::FormatInterpreterToken;
use crate::FormatNodeFields;
use rome_js_syntax::JsModule;
use rome_js_syntax::JsModuleFields;

impl FormatNodeFields<JsModule> for FormatNodeRule<JsModule> {
    fn format_fields(node: &JsModule, f: &mut JsFormatter) -> FormatResult<()> {
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

        f.join_with(&hard_line_break())
            .entries(items.try_format_nodes())
            .finish()?;

        write!(
            f,
            [
                f.format_replaced(&eof_token?, &empty_element()),
                hard_line_break()
            ]
        )
    }
}
