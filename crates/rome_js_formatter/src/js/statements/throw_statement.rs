use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::utils::FormatWithSemicolon;
use crate::FormatNodeFields;
use rome_js_syntax::JsThrowStatement;
use rome_js_syntax::JsThrowStatementFields;

impl FormatNodeFields<JsThrowStatement> for FormatNodeRule<JsThrowStatement> {
    fn fmt_fields(node: &JsThrowStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsThrowStatementFields {
            throw_token,
            argument,
            semicolon_token,
        } = node.as_fields();

        let throw_token = throw_token.format();
        let exception = argument.format();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args!(throw_token, space_token(), exception),
                semicolon_token.as_ref()
            )]
        )
    }
}
