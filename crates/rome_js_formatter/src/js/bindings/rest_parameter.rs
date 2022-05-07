use crate::prelude::*;

use rome_js_syntax::JsRestParameter;
use rome_js_syntax::JsRestParameterFields;

impl FormatNode for JsRestParameter {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsRestParameterFields {
            dotdotdot_token,
            binding,
            type_annotation,
        } = self.as_fields();

        formatted![
            formatter,
            dotdotdot_token.format(formatter)?,
            binding.format(formatter)?,
            type_annotation
        ]
    }
}
