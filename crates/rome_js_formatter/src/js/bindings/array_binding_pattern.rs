use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsArrayBindingPattern;
use rome_js_syntax::JsArrayBindingPatternFields;

impl FormatNodeFields<JsArrayBindingPattern> for FormatNodeRule<JsArrayBindingPattern> {
    fn format_fields(
        node: &JsArrayBindingPattern,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsArrayBindingPatternFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = node.as_fields();

        formatter.format_delimited_soft_block_indent(
            &l_brack_token?,
            formatted![formatter, [elements.format()]]?,
            &r_brack_token?,
        )
    }
}
