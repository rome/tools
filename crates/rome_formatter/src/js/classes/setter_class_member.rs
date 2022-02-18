use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
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

        Ok(format_elements![
            set_token.format(formatter)?,
            space_token(),
            name.format(formatter)?,
            l_paren_token.format(formatter)?,
            parameter.format(formatter)?,
            r_paren_token.format(formatter)?,
            space_token(),
            body.format(formatter)?,
        ])
    }
}
