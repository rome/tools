use crate::formatter_traits::FormatOptionalTokenAndNode;
use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, hard_group_elements, space_token, token, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};
use rslint_syntax::TsConstructorSignatureClassMember;
use rslint_syntax::TsConstructorSignatureClassMemberFields;

impl ToFormatElement for TsConstructorSignatureClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsConstructorSignatureClassMemberFields {
            modifiers,
            name,
            parameters,
            semicolon_token,
        } = self.as_fields();

        Ok(hard_group_elements(format_elements![
            modifiers.format(formatter)?,
            space_token(),
            name.format(formatter)?,
            parameters.format(formatter)?,
            semicolon_token.format_or(formatter, || token(";"))?
        ]))
    }
}
