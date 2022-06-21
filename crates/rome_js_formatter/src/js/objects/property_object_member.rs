use crate::prelude::*;
use crate::utils::JsAnyAssignmentLike;

use rome_formatter::write;
use rome_js_syntax::JsPropertyObjectMember;

#[derive(Debug, Clone, Default)]
pub struct FormatJsPropertyObjectMember;

impl FormatNodeRule<JsPropertyObjectMember> for FormatJsPropertyObjectMember {
    fn fmt_fields(node: &JsPropertyObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [JsAnyAssignmentLike::from(node.clone())]]
    }
}
