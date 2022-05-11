use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsArrayAssignmentPattern;
use rome_js_syntax::JsArrayAssignmentPatternFields;

impl FormatNodeFields<JsArrayAssignmentPattern> for FormatNodeRule<JsArrayAssignmentPattern> {
    fn format_fields(
        node: &JsArrayAssignmentPattern,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        let JsArrayAssignmentPatternFields {
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
