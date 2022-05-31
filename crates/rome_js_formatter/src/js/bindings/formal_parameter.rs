use crate::prelude::*;
use crate::utils::format_initializer_clause;

use crate::FormatNodeFields;
use rome_js_syntax::JsFormalParameter;
use rome_js_syntax::JsFormalParameterFields;

impl FormatNodeFields<JsFormalParameter> for FormatNodeRule<JsFormalParameter> {
    fn format_fields(
        node: &JsFormalParameter,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsFormalParameterFields {
            binding,
            question_mark_token,
            type_annotation,
            initializer,
        } = node.as_fields();

        let initializer = format_initializer_clause(formatter, initializer)?;

        formatted![
            formatter,
            [
                binding.format(),
                question_mark_token.format(),
                type_annotation.format(),
                initializer
            ]
        ]
    }
}
