use crate::{
    format_elements,
    formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode},
    hard_group_elements, space_token, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::{TsGetterSignatureClassMember, TsGetterSignatureClassMemberFields};

impl ToFormatElement for TsGetterSignatureClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsGetterSignatureClassMemberFields {
            access_modifier,
            abstract_token,
            get_token,
            name,
            l_paren_token,
            r_paren_token,
            return_type,
            semicolon_token,
        } = self.as_fields();

        let access_modifier = access_modifier
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;
        let abstract_token = abstract_token.format(formatter)?;
        let get_token = get_token.format(formatter)?;
        let name = name.format(formatter)?;
        let l_paren_token = l_paren_token.format(formatter)?;
        let r_paren_token = r_paren_token.format(formatter)?;
        let return_type = return_type.format_or_empty(formatter)?;
        let semicolon_token = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(hard_group_elements(format_elements![
            access_modifier,
            abstract_token,
            space_token(),
            get_token,
            space_token(),
            name,
            l_paren_token,
            r_paren_token,
            return_type,
            semicolon_token
        ]))
    }
}
