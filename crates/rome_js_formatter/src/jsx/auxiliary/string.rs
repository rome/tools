use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsxString;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxString;

impl FormatNodeRule<JsxString> for FormatJsxString {
    fn fmt_fields(&self, node: &JsxString, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [node.value_token().format()]]
    }
}
