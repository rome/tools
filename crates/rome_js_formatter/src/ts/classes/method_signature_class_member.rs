use crate::prelude::*;
use crate::utils::format_with_semicolon;
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

        let async_token =
            async_token.with_or_empty(|token| formatted![formatter, token, space_token()]);
        let name = name.format(formatter)?;
        let parameters = parameters.format(formatter)?;

        Ok(hard_group_elements(format_with_semicolon(
            formatter,
            formatted![
                formatter,
                modifiers.format(formatter)?,
                async_token,
                space_token(),
                name,
                question_mark_token,
                type_parameters,
                parameters,
                return_type_annotation,
            ]?,
            semicolon_token,
        )?))
    }
}
