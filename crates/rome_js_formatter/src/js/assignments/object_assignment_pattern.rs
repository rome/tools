use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::JsObjectAssignmentPattern;
use rome_js_syntax::JsObjectAssignmentPatternFields;

impl FormatNodeFields<JsObjectAssignmentPattern> for FormatNodeRule<JsObjectAssignmentPattern> {
    fn format_fields(
        node: &JsObjectAssignmentPattern,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        let JsObjectAssignmentPatternFields {
            l_curly_token,
            properties,
            r_curly_token,
        } = node.as_fields();

        formatter.format_delimited_soft_block_spaces(
            &l_curly_token?,
            formatted![formatter, properties.format()]?,
            &r_curly_token?,
        )
    }
}
