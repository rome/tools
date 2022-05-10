use crate::utils::format_with_semicolon;
use crate::{
    format_traits::FormatOptional, hard_group_elements, space_token, Format,
    FormatElement, FormatNode, Formatter,
};
use rome_formatter::FormatResult;
use rome_js_syntax::{TsMethodSignatureClassMember, TsMethodSignatureClassMemberFields};

impl FormatNode for TsMethodSignatureClassMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsMethodSignatureClassMemberFields {
            modifiers,
            async_token,
            name,
            question_mark_token,
            type_parameters,
            parameters,
            return_type_annotation,
            semicolon_token,
        } = self.as_fields();

        let async_token = async_token.format_with_or_empty(formatter, |token| {
            formatted![formatter, token, space_token()]
        })?;
        let name = name.format(formatter)?;
        let type_parameters = type_parameters.format_or_empty(formatter)?;
        let parameters = parameters.format(formatter)?;
        let return_type_annotation = return_type_annotation.format_or_empty(formatter)?;

        Ok(hard_group_elements(format_with_semicolon(
            formatter,
            formatted![
                formatter,
                modifiers.format(formatter)?,
                async_token,
                space_token(),
                name,
                question_mark_token.format_or_empty(formatter)?,
                type_parameters,
                parameters,
                return_type_annotation,
            ]?,
            semicolon_token,
        )?))
    }
}
