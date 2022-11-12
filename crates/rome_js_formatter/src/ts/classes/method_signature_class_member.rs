use crate::prelude::*;
use crate::utils::FormatSemicolon;

use crate::js::classes::method_class_member::FormatJsAnyMethodMember;
use rome_formatter::write;
use rome_js_syntax::TsMethodSignatureClassMember;

#[derive(Debug, Clone, Default)]
pub struct FormatTsMethodSignatureClassMember;

impl FormatNodeRule<TsMethodSignatureClassMember> for FormatTsMethodSignatureClassMember {
    fn fmt_fields(
        &self,
        node: &TsMethodSignatureClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let modifiers = node.modifiers();

        if !modifiers.is_empty() {
            write!(f, [modifiers.format(), space()])?;
        }

        write!(
            f,
            [
                FormatJsAnyMethodMember::from(node.clone()),
                FormatSemicolon::new(node.semicolon_token().as_ref())
            ]
        )
    }
}
