use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsxString;

impl FormatNodeFields<JsxString> for FormatNodeRule<JsxString> {
    fn fmt_fields(node: &JsxString, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [node.value_token().format()]]
    }
}
