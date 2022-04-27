use crate::format_traits::FormatOptional;
use rome_formatter::FormatResult;

use crate::utils::{format_property_name, format_with_semicolon, PropertyNameCheckMode};
use crate::{format_elements, space_token, Format, FormatElement, FormatNode, Formatter};

use rome_js_syntax::JsPropertyClassMember;
use rome_js_syntax::JsPropertyClassMemberFields;

impl FormatNode for JsPropertyClassMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPropertyClassMemberFields {
            modifiers,
            name,
            property_annotation,
            value,
            semicolon_token,
        } = self.as_fields();

        let property_annotation = property_annotation.format_or_empty(formatter)?;

        let init =
            value.format_with_or_empty(formatter, |node| format_elements![space_token(), node])?;

        format_with_semicolon(
            formatter,
            format_elements![
                modifiers.format(formatter)?,
                space_token(),
                format_property_name(name?, formatter, PropertyNameCheckMode::Alphanumeric)?,
                property_annotation,
                init,
            ],
            semicolon_token,
        )
    }
}
