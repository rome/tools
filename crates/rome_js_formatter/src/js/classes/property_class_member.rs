use crate::format_traits::FormatOptional;
use rome_formatter::FormatResult;

use crate::utils::format_with_semicolon;
use crate::{formatted, space_token, Format, FormatElement, FormatNode, Formatter};

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

        let init = value
            .format_with_or_empty(formatter, |node| formatted![formatter, space_token(), node])?;

        format_with_semicolon(
            formatter,
            formatted![
                formatter,
                modifiers.format(formatter)?,
                space_token(),
                name.format(formatter)?,
                property_annotation,
                init,
            ]?,
            semicolon_token,
        )
    }
}
