use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{JsxSpreadAttribute, JsxSpreadAttributeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxSpreadAttribute;

impl FormatNodeRule<JsxSpreadAttribute> for FormatJsxSpreadAttribute {
    fn fmt_fields(&self, node: &JsxSpreadAttribute, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxSpreadAttributeFields {
            l_curly_token,
            dotdotdot_token,
            argument,
            r_curly_token,
        } = node.as_fields();

        write![
            f,
            [
                l_curly_token.format(),
                dotdotdot_token.format(),
                argument.format(),
                r_curly_token.format(),
            ]
        ]
    }
}
