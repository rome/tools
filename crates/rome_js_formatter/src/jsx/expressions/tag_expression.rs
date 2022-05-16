use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::JsxTagExpression;

impl FormatNodeFields<JsxTagExpression> for FormatNodeRule<JsxTagExpression> {
    fn format_fields(
        node: &JsxTagExpression,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        formatted![formatter, [node.tag().format()]]
    }
}
