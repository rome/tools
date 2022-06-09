use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{JsxName, JsxNameFields};

impl FormatNodeFields<JsxName> for FormatNodeRule<JsxName> {
    fn fmt_fields(node: &JsxName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxNameFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }
}
