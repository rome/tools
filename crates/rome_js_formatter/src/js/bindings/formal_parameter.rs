use crate::prelude::*;
use rome_formatter::write;

use crate::utils::FormatInitializerClause;

use rome_js_syntax::JsFormalParameterFields;
use rome_js_syntax::{JsAnyBindingPattern, JsFormalParameter};

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

        if let JsAnyBindingPattern::JsObjectBindingPattern(_) = node.binding()? {
            write![f, [group(&content)]]?;
        } else {
            write![f, [content]]?;
        }

        write![f, [FormatInitializerClause::new(initializer.as_ref())]]
    }
}
