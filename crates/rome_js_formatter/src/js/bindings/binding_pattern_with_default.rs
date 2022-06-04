use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsBindingPatternWithDefault;
use rome_js_syntax::JsBindingPatternWithDefaultFields;

impl FormatNodeFields<JsBindingPatternWithDefault> for FormatNodeRule<JsBindingPatternWithDefault> {
    fn fmt_fields(node: &JsBindingPatternWithDefault, f: &mut JsFormatter) -> FormatResult<()> {
        let JsBindingPatternWithDefaultFields {
            pattern,
            eq_token,
            default,
        } = node.as_fields();

        write![
            f,
            [
                pattern.format(),
                space_token(),
                eq_token.format(),
                space_token(),
                default.format()
            ]
        ]
    }
}
