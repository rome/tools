use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsxTagExpression;

impl FormatNodeFields<JsxTagExpression> for FormatNodeRule<JsxTagExpression> {
    fn fmt_fields(node: &JsxTagExpression, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [node.tag().format()]]
    }
}
