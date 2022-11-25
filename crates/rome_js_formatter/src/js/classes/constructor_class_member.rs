use crate::prelude::*;

use crate::js::classes::method_class_member::FormatJsAnyMethodMember;
use rome_formatter::write;
use rome_js_syntax::JsConstructorClassMember;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsConstructorClassMember;

impl FormatNodeRule<JsConstructorClassMember> for FormatJsConstructorClassMember {
    fn fmt_fields(&self, node: &JsConstructorClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        write![
            f,
            [
                node.modifiers().format(),
                space(),
                FormatJsAnyMethodMember::from(node.clone())
            ]
        ]
    }
}
