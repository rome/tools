use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsBindingPatternWithDefault;
use rome_js_syntax::JsBindingPatternWithDefaultFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsBindingPatternWithDefault;

impl FormatNodeRule<JsBindingPatternWithDefault> for FormatJsBindingPatternWithDefault {
    fn fmt_fields(
        &self,
        node: &JsBindingPatternWithDefault,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
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
