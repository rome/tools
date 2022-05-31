use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsObjectBindingPatternRest;
use rome_js_syntax::JsObjectBindingPatternRestFields;

impl FormatNodeFields<JsObjectBindingPatternRest> for FormatNodeRule<JsObjectBindingPatternRest> {
    fn format_fields(
        node: &JsObjectBindingPatternRest,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsObjectBindingPatternRestFields {
            dotdotdot_token,
            binding,
        } = node.as_fields();

        formatted![formatter, [dotdotdot_token.format(), binding.format(),]]
    }
}
