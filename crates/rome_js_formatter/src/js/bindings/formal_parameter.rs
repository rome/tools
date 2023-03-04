use crate::prelude::*;
use rome_formatter::write;

use crate::utils::FormatInitializerClause;

use crate::js::bindings::parameters::{should_hug_function_parameters, FormatAnyJsParameters};
use rome_js_syntax::JsFormalParameter;
use rome_js_syntax::JsFormalParameterFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsFormalParameter;

impl FormatNodeRule<JsFormalParameter> for FormatJsFormalParameter {
    fn fmt_fields(&self, node: &JsFormalParameter, f: &mut JsFormatter) -> FormatResult<()> {
        let JsFormalParameterFields {
            binding,
            question_mark_token,
            type_annotation,
            initializer,
        } = node.as_fields();

        let content = format_with(|f| {
            write![
                f,
                [
                    binding.format(),
                    question_mark_token.format(),
                    type_annotation.format()
                ]
            ]
        });

        let is_hug_parameter = node
            .syntax()
            .grand_parent()
            .and_then(FormatAnyJsParameters::cast)
            .map_or(false, |parameters| {
                should_hug_function_parameters(&parameters, f.comments()).unwrap_or(false)
            });

        if is_hug_parameter {
            write![f, [content]]?;
        } else {
            write![f, [group(&content)]]?;
        }

        write![f, [FormatInitializerClause::new(initializer.as_ref())]]
    }
}
