use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsRestParameter;
use rome_js_syntax::JsRestParameterFields;

impl FormatNodeFields<JsRestParameter> for FormatNodeRule<JsRestParameter> {
    fn format_fields(
        node: &JsRestParameter,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsRestParameterFields {
            dotdotdot_token,
            binding,
            type_annotation,
        } = node.as_fields();

        formatted![
            formatter,
            [
                dotdotdot_token.format(),
                binding.format(),
                type_annotation.format(),
            ]
        ]
    }
}
