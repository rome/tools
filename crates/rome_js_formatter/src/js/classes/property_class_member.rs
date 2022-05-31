use crate::prelude::*;

use crate::utils::format_with_semicolon;

use crate::FormatNodeFields;
use rome_js_syntax::JsPropertyClassMember;
use rome_js_syntax::JsPropertyClassMemberFields;

impl FormatNodeFields<JsPropertyClassMember> for FormatNodeRule<JsPropertyClassMember> {
    fn format_fields(
        node: &JsPropertyClassMember,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsPropertyClassMemberFields {
            modifiers,
            name,
            property_annotation,
            value,
            semicolon_token,
        } = node.as_fields();

        format_with_semicolon(
            formatter,
            formatted![
                formatter,
                [
                    modifiers.format(),
                    space_token(),
                    name.format(),
                    property_annotation.format(),
                    value
                        .format()
                        .with_or_empty(|node| formatted![formatter, [space_token(), node]]),
                ]
            ]?,
            semicolon_token,
        )
    }
}
