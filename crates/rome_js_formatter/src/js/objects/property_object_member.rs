use crate::{format_elements, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use crate::utils::format_property_name;
use rome_js_syntax::JsPropertyObjectMember;
use rome_js_syntax::JsPropertyObjectMemberFields;

impl FormatNode for JsPropertyObjectMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPropertyObjectMemberFields {
            name,
            colon_token,
            value,
        } = self.as_fields();

        let key = format_property_name(
            name?,
            formatter,
            crate::utils::PropertyNameCheckMode::Alphanumeric,
        )?;
        let colon = colon_token.format(formatter)?;
        let value = value.format(formatter)?;
        Ok(format_elements![key, colon, space_token(), value])
    }
}
