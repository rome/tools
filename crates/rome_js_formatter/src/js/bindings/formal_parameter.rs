use crate::format_traits::FormatOptional;
use crate::utils::format_initializer_clause;
use crate::{format_elements, Format, FormatElement, FormatNode, Formatter};
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

        let type_annotation = type_annotation.format_or_empty(formatter)?;
        let initializer = format_initializer_clause(formatter, initializer)?;

        Ok(format_elements![
            binding.format(formatter)?,
            question_mark_token.format_or_empty(formatter)?,
            type_annotation,
            initializer
        ])
    }
}
