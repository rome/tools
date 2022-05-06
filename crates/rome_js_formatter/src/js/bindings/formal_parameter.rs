
use crate::utils::format_initializer_clause;
use crate::{formatted, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::JsFormalParameter;
use rome_js_syntax::JsFormalParameterFields;

impl FormatNode for JsFormalParameter {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsFormalParameterFields {
            binding,
            question_mark_token,
            type_annotation,
            initializer,
        } = self.as_fields();

        let initializer = format_initializer_clause(formatter, initializer)?;

        formatted![
            formatter,
            binding.format(formatter)?,
            question_mark_token,
            type_annotation,
            initializer
        ]
    }
}
