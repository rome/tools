use crate::prelude::*;
use crate::utils::JsAnyAssignmentLike;

use rome_formatter::write;
use rome_js_syntax::JsPropertyObjectMember;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsPropertyObjectMember;

impl FormatNodeRule<JsPropertyObjectMember> for FormatJsPropertyObjectMember {
    fn fmt_fields(&self, node: &JsPropertyObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [JsAnyAssignmentLike::from(node.clone())]]
    }
}
