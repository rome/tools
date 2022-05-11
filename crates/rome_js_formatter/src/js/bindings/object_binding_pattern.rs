use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsObjectBindingPattern;
use rome_js_syntax::JsObjectBindingPatternFields;

impl FormatNodeFields<JsObjectBindingPattern> for FormatNodeRule<JsObjectBindingPattern> {
    fn format_fields(
        node: &JsObjectBindingPattern,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        let JsObjectBindingPatternFields {
            l_curly_token,
            properties,
            r_curly_token,
        } = node.as_fields();

        formatter.format_delimited_soft_block_spaces(
            &l_curly_token?,
            formatted![formatter, [properties.format()]]?,
            &r_curly_token?,
        )
    }
}
