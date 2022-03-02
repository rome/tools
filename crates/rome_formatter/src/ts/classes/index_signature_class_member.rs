use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsIndexSignatureClassMember;
use rslint_parser::ast::TsIndexSignatureClassMemberFields;

impl ToFormatElement for TsIndexSignatureClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsIndexSignatureClassMemberFields {
            modifiers,
            l_brack_token,
            parameter,
            r_brack_token,
            type_annotation,
            semicolon_token,
        } = self.as_fields();

        let semicolon_token = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(format_elements![
            modifiers.format(formatter)?,
            space_token(),
            l_brack_token.format(formatter)?,
            parameter.format(formatter)?,
            r_brack_token.format(formatter)?,
            type_annotation.format(formatter)?,
            semicolon_token
        ])
    }
}
