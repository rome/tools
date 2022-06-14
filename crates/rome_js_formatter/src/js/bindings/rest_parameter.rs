use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsRestParameter;
use rome_js_syntax::JsRestParameterFields;

impl FormatNodeFields<JsRestParameter> for FormatNodeRule<JsRestParameter> {
    fn fmt_fields(node: &JsRestParameter, f: &mut JsFormatter) -> FormatResult<()> {
        let JsRestParameterFields {
            dotdotdot_token,
            binding,
            type_annotation,
        } = node.as_fields();

        write![
            f,
            [
                dotdotdot_token.format(),
                binding.format(),
                type_annotation.format(),
            ]
        ]
    }
}
