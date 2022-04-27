use crate::format_traits::FormatOptional;
use crate::{
    format_elements, hard_group_elements, space_token, utils::format_property_name, Format,
    FormatElement, FormatNode, Formatter,
};
use rome_formatter::FormatResult;

use crate::utils::PropertyNameCheckMode;
use rome_js_syntax::JsGetterClassMember;
use rome_js_syntax::JsGetterClassMemberFields;

impl FormatNode for JsGetterClassMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsGetterClassMemberFields {
            modifiers,
            get_token,
            name,
            l_paren_token,
            r_paren_token,
            return_type,
            body,
        } = self.as_fields();

        Ok(hard_group_elements(format_elements![
            modifiers.format(formatter)?,
            space_token(),
            get_token.format(formatter)?,
            space_token(),
            format_property_name(name?, formatter, PropertyNameCheckMode::Alphanumeric)?,
            l_paren_token.format(formatter)?,
            r_paren_token.format(formatter)?,
            return_type.format_or_empty(formatter)?,
            space_token(),
            body.format(formatter)?
        ]))
    }
}
