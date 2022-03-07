use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, hard_group_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rome_js_syntax::JsSetterClassMember;
use rome_js_syntax::JsSetterClassMemberFields;

impl ToFormatElement for JsSetterClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsSetterClassMemberFields {
            modifiers,
            set_token,
            name,
            l_paren_token,
            parameter,
            r_paren_token,
            body,
        } = self.as_fields();

        Ok(hard_group_elements(format_elements![
            modifiers.format(formatter)?,
            space_token(),
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
