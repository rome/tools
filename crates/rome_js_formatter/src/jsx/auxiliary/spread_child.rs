use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::{JsxSpreadChild, JsxSpreadChildFields};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxSpreadChild;

impl FormatNodeRule<JsxSpreadChild> for FormatJsxSpreadChild {
    fn fmt_fields(&self, node: &JsxSpreadChild, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxSpreadChildFields {
            l_curly_token,
            dotdotdot_token,
            expression,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_curly_token.format(),
                dotdotdot_token.format(),
                expression.format(),
                r_curly_token.format()
            ]
        )
    }
}
