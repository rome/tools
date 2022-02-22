use crate::{
    format_elements,
    formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode},
    hard_group_elements, space_token, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::{TsAbstractGetterClassMember, TsAbstractGetterClassMemberFields};
impl ToFormatElement for TsAbstractGetterClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsAbstractGetterClassMemberFields {
            access_modifier,
            abstract_token,
            get_token,
            name,
            l_paren_token,
            r_paren_token,
            return_type,
        } = self.as_fields();

        Ok(hard_group_elements(format_elements![
            access_modifier
                .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?,
            abstract_token
                .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?,
            get_token.format(formatter)?,
            space_token(),
            name.format(formatter)?,
            l_paren_token.format(formatter)?,
            r_paren_token.format(formatter)?,
            return_type.format_or_empty(formatter)?,
            token(";")
        ]))
    }
}
