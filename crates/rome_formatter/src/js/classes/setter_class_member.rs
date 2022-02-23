use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, hard_group_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rslint_parser::ast::JsSetterClassMember;
use rslint_parser::ast::JsSetterClassMemberFields;

impl ToFormatElement for JsSetterClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsSetterClassMemberFields {
            access_modifier,
            static_token,
            abstract_token,
            set_token,
            name,
            l_paren_token,
            parameter,
            r_paren_token,
            body,
        } = self.as_fields();
        let access_modifier = access_modifier.format_with_or_empty(formatter, |element| {
            format_elements![element, space_token()]
        })?;
        let static_token = static_token.format_with_or_empty(formatter, |element| {
            format_elements![element, space_token()]
        })?;
        let abstract_token = abstract_token.format_with_or_empty(formatter, |element| {
            format_elements![element, space_token()]
        })?;

        Ok(hard_group_elements(format_elements![
            access_modifier,
            static_token,
            abstract_token,
            set_token.format(formatter)?,
            space_token(),
            name.format(formatter)?,
            l_paren_token.format(formatter)?,
            parameter.format(formatter)?,
            r_paren_token.format(formatter)?,
            space_token(),
            body.format(formatter)?,
        ]))
    }
}
