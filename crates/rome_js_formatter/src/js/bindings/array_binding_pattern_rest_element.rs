use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsArrayBindingPatternRestElement;
use rome_js_syntax::JsArrayBindingPatternRestElementFields;

impl FormatNodeFields<JsArrayBindingPatternRestElement>
    for FormatNodeRule<JsArrayBindingPatternRestElement>
{
    fn fmt_fields(
        node: &JsArrayBindingPatternRestElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsArrayBindingPatternRestElementFields {
            dotdotdot_token,
            pattern,
        } = node.as_fields();

        write![f, [dotdotdot_token.format(), pattern.format()]]
    }
}
