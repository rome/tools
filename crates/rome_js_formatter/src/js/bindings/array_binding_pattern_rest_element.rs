use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsArrayBindingPatternRestElement;
use rome_js_syntax::JsArrayBindingPatternRestElementFields;

impl FormatNodeFields<JsArrayBindingPatternRestElement>
    for FormatNodeRule<JsArrayBindingPatternRestElement>
{
    fn format_fields(
        node: &JsArrayBindingPatternRestElement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsArrayBindingPatternRestElementFields {
            dotdotdot_token,
            pattern,
        } = node.as_fields();

        formatted![formatter, [dotdotdot_token.format(), pattern.format(),]]
    }
}
