use crate::prelude::*;
use crate::utils::{FormatWithSemicolon, JsAnyAssignmentLike};
use rome_formatter::write;
use rome_js_syntax::JsPropertyClassMember;

#[derive(Debug, Clone, Default)]
pub struct FormatJsPropertyClassMember;

impl FormatNodeRule<JsPropertyClassMember> for FormatJsPropertyClassMember {
    fn fmt_fields(&self, node: &JsPropertyClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        let semicolon_token = node.semicolon_token();
        let body = format_with(|f| write!(f, [JsAnyAssignmentLike::from(node.clone())]));
        write!(
            f,
            [FormatWithSemicolon::new(&body, semicolon_token.as_ref())]
        )
    }
}
