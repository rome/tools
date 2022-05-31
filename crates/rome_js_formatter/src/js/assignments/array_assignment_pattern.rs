use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsArrayAssignmentPattern;
use rome_js_syntax::JsArrayAssignmentPatternFields;

impl FormatNodeFields<JsArrayAssignmentPattern> for FormatNodeRule<JsArrayAssignmentPattern> {
    fn format_fields(
        node: &JsArrayAssignmentPattern,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsArrayAssignmentPatternFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = node.as_fields();

        formatter
            .delimited(
                &l_brack_token?,
                formatted![formatter, [elements.format()]]?,
                &r_brack_token?,
            )
            .soft_block_indent()
            .finish()
    }
}
