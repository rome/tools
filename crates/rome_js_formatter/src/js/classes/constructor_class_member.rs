use crate::{
    format_elements, hard_group_elements, space_token, utils::format_property_name, Format,
    FormatElement, FormatNode, Formatter,
};
use rome_formatter::FormatResult;

use crate::utils::PropertyNameCheckMode;
use rome_js_syntax::JsConstructorClassMember;
use rome_js_syntax::JsConstructorClassMemberFields;

impl FormatNode for JsConstructorClassMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsConstructorClassMemberFields {
            modifiers,
            name,
            parameters,
            body,
        } = self.as_fields();

        Ok(hard_group_elements(format_elements![
            modifiers.format(formatter)?,
            space_token(),
            format_property_name(name?, formatter, PropertyNameCheckMode::Alphanumeric)?,
            parameters.format(formatter)?,
            space_token(),
            body.format(formatter)?
        ]))
    }
}
