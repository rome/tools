use crate::{format_elements, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::group_elements;
use rome_formatter::soft_line_break_or_space;
use rome_formatter::FormatResult;

use rome_js_syntax::JsPropertyObjectMember;
use rome_js_syntax::JsPropertyObjectMemberFields;

impl FormatNode for JsPropertyObjectMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPropertyObjectMemberFields {
            name,
            colon_token,
            value,
        } = self.as_fields();

        let key = name.format(formatter)?;
        let key = format_member_name(name?, formatter, crate::utils::MemberContext::Member)?;
        let colon = colon_token.format(formatter)?;
        let value = value.format(formatter)?;
        Ok(group_elements(format_elements![
            key,
            colon,
            soft_line_break_or_space(),
            value
        ]))
    }
}
