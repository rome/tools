use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsRestParameter;
use rome_js_syntax::JsRestParameterFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsRestParameter;

impl FormatNodeRule<JsRestParameter> for FormatJsRestParameter {
    fn fmt_fields(&self, node: &JsRestParameter, f: &mut JsFormatter) -> FormatResult<()> {
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
