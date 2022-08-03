use crate::prelude::*;

use crate::js::classes::method_class_member::FormatJsAnyMethodMember;
use rome_js_syntax::JsMethodObjectMember;

#[derive(Debug, Clone, Default)]
pub struct FormatJsMethodObjectMember;

impl FormatNodeRule<JsMethodObjectMember> for FormatJsMethodObjectMember {
    fn fmt_fields(&self, node: &JsMethodObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        FormatJsAnyMethodMember::from(node.clone()).fmt(f)
    }
}
