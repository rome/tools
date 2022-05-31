use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::JsObjectAssignmentPattern;
use rome_js_syntax::JsObjectAssignmentPatternFields;

impl FormatNodeFields<JsObjectAssignmentPattern> for FormatNodeRule<JsObjectAssignmentPattern> {
    fn format_fields(
        node: &JsObjectAssignmentPattern,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsObjectAssignmentPatternFields {
            l_curly_token,
            properties,
            r_curly_token,
        } = node.as_fields();

        formatter
            .delimited(
                &l_curly_token?,
                formatted![formatter, [properties.format()]]?,
                &r_curly_token?,
            )
            .soft_block_spaces()
            .finish()
    }
}
