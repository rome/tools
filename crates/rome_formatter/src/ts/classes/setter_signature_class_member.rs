use crate::{
    format_elements,
    formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode},
    hard_group_elements, space_token, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_syntax::{TsSetterSignatureClassMember, TsSetterSignatureClassMemberFields};

impl ToFormatElement for TsSetterSignatureClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsSetterSignatureClassMemberFields {
            modifiers,
            set_token,
            name,
            l_paren_token,
            parameter,
            r_paren_token,
            semicolon_token,
        } = self.as_fields();

        let set_token = set_token.format(formatter)?;
        let name = name.format(formatter)?;
        let l_paren_token = l_paren_token.format(formatter)?;
        let parameters = parameter.format(formatter)?;
        let r_paren_token = r_paren_token.format(formatter)?;
        let semicolon_token = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(hard_group_elements(format_elements![
            modifiers.format(formatter)?,
            space_token(),
            set_token,
            space_token(),
            name,
            l_paren_token,
            parameters,
            r_paren_token,
            semicolon_token
        ]))
    }
}
