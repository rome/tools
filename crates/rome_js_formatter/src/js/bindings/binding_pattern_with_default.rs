use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsBindingPatternWithDefault;
use rome_js_syntax::JsBindingPatternWithDefaultFields;

impl FormatNodeFields<JsBindingPatternWithDefault> for FormatNodeRule<JsBindingPatternWithDefault> {
    fn format_fields(
        node: &JsBindingPatternWithDefault,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsBindingPatternWithDefaultFields {
            pattern,
            eq_token,
            default,
        } = node.as_fields();

        formatted![
            formatter,
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
