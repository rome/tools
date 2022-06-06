use crate::prelude::*;
use rome_formatter::write;

use crate::utils::FormatInitializerClause;
use crate::FormatNodeFields;
use rome_js_syntax::JsFormalParameter;
use rome_js_syntax::JsFormalParameterFields;

impl FormatNodeFields<JsFormalParameter> for FormatNodeRule<JsFormalParameter> {
    fn fmt_fields(node: &JsFormalParameter, f: &mut JsFormatter) -> FormatResult<()> {
        let JsFormalParameterFields {
            binding,
            question_mark_token,
            type_annotation,
            initializer,
        } = node.as_fields();

        write![
            f,
            [
                binding.format(),
                question_mark_token.format(),
                type_annotation.format(),
                FormatInitializerClause::new(initializer.as_ref())
            ]
        ]
    }
}
