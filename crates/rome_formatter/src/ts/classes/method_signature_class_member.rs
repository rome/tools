use crate::{
    format_elements,
    formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode},
    hard_group_elements, space_token, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::{TsMethodSignatureClassMember, TsMethodSignatureClassMemberFields};

impl ToFormatElement for TsMethodSignatureClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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

        let async_token = async_token
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;
        let name = name.format(formatter)?;
        let type_parameters = type_parameters.format_or_empty(formatter)?;
        let parameters = parameters.format(formatter)?;
        let return_type_annotation = return_type_annotation.format_or_empty(formatter)?;
        let semicolon_token = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(hard_group_elements(format_elements![
            modifiers.format(formatter)?,
            async_token,
            space_token(),
            name,
            question_mark_token.format_or_empty(formatter)?,
            type_parameters,
            parameters,
            return_type_annotation,
            semicolon_token
        ]))
    }
}
