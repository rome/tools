use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsIndexSignatureClassMember;
use rslint_parser::ast::TsIndexSignatureClassMemberFields;

impl ToFormatElement for TsIndexSignatureClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsIndexSignatureClassMemberFields {
            static_token,
            readonly_token,
            l_brack_token,
            parameter,
            r_brack_token,
            type_annotation,
            semicolon_token,
        } = self.as_fields();

        let static_token = static_token.format_with_or_empty(formatter, |static_token| {
            format_elements![static_token, space_token()]
        })?;
        let readonly_token = readonly_token.format_with_or_empty(formatter, |readonly_token| {
            format_elements![readonly_token, space_token()]
        })?;
        let semicolon_token = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(format_elements![
            static_token,
            readonly_token,
            l_brack_token.format(formatter)?,
            parameter.format(formatter)?,
            r_brack_token.format(formatter)?,
            type_annotation.format(formatter)?,
            semicolon_token
        ])
    }
}
