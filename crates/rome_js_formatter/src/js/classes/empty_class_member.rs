use crate::prelude::*;

use rome_js_syntax::JsEmptyClassMember;
use rome_js_syntax::JsEmptyClassMemberFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsEmptyClassMember;

impl FormatNodeRule<JsEmptyClassMember> for FormatJsEmptyClassMember {
    fn fmt_fields(&self, node: &JsEmptyClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        let JsEmptyClassMemberFields { semicolon_token } = node.as_fields();

        format_removed(&semicolon_token?).fmt(f)
    }
}
