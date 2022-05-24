use crate::prelude::*;

use crate::utils::{format_member_name, MemberContext};
use crate::FormatNodeFields;
use rome_js_syntax::JsPropertyObjectMember;
use rome_js_syntax::JsPropertyObjectMemberFields;

impl FormatNodeFields<JsPropertyObjectMember> for FormatNodeRule<JsPropertyObjectMember> {
    fn format_fields(
        node: &JsPropertyObjectMember,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsPropertyObjectMemberFields {
            name,
            colon_token,
            value,
        } = node.as_fields();

        let key = format_member_name(name?, formatter, MemberContext::Member);
        let colon = colon_token.format();
        let value = value.format();
        formatted![formatter, [key, colon, space_token(), value]]
    }
}
