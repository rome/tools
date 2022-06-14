use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsObjectBindingPatternRest;
use rome_js_syntax::JsObjectBindingPatternRestFields;

impl FormatNodeFields<JsObjectBindingPatternRest> for FormatNodeRule<JsObjectBindingPatternRest> {
    fn fmt_fields(node: &JsObjectBindingPatternRest, f: &mut JsFormatter) -> FormatResult<()> {
        let JsObjectBindingPatternRestFields {
            dotdotdot_token,
            binding,
        } = node.as_fields();

        write![f, [dotdotdot_token.format(), binding.format(),]]
    }
}
