use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsExpressionSnipped;
use rome_js_syntax::JsExpressionSnippedFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsExpressionSnipped;

impl FormatNodeRule<JsExpressionSnipped> for FormatJsExpressionSnipped {
    fn fmt_fields(&self, node: &JsExpressionSnipped, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExpressionSnippedFields {
            expression,
            eof_token,
        } = node.as_fields();

        write![f, [expression.format(), format_removed(&eof_token?),]]
    }
}
