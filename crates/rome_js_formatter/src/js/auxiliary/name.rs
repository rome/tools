use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsName;
use rome_js_syntax::JsNameFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsName;

impl FormatNodeRule<JsName> for FormatJsName {
    fn fmt_fields(&self, node: &JsName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsNameFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }
}
