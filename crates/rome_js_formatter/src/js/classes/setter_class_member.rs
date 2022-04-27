use crate::utils::PropertyNameCheckMode;
use crate::{
    format_elements, hard_group_elements, space_token, utils::format_property_name, Format,
    FormatElement, FormatNode, Formatter,
};
use rome_formatter::FormatResult;
use rome_js_syntax::JsSetterClassMember;
use rome_js_syntax::JsSetterClassMemberFields;

impl FormatNode for JsSetterClassMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
            format_property_name(name?, formatter, PropertyNameCheckMode::Alphanumeric)?,
            l_paren_token.format(formatter)?,
            parameter.format(formatter)?,
            r_paren_token.format(formatter)?,
            space_token(),
            body.format(formatter)?,
        ]))
    }
}
