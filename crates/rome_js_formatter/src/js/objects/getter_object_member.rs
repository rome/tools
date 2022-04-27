use crate::format_traits::FormatOptional;
use rome_formatter::FormatResult;

use crate::{
    format_elements, hard_group_elements, space_token, Format, FormatElement, FormatNode, Formatter,
};
use rome_js_syntax::JsGetterObjectMember;
use rome_js_syntax::JsGetterObjectMemberFields;

impl FormatNode for JsGetterObjectMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsGetterObjectMemberFields {
            get_token,
            name,
            l_paren_token,
            r_paren_token,
            return_type,
            body,
        } = self.as_fields();

        Ok(hard_group_elements(format_elements![
            get_token.format(formatter)?,
            space_token(),
            name.format(formatter)?,
            l_paren_token.format(formatter)?,
            r_paren_token.format(formatter)?,
            return_type.format_or_empty(formatter)?,
            space_token(),
            body.format(formatter)?
        ]))
    }
}
