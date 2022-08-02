use crate::prelude::*;
use crate::utils::FormatWithSemicolon;

use crate::js::classes::method_class_member::FormatMethodMember;
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
        let format_inner = format_with(|f| {
            let modifiers = node.modifiers();

            if !modifiers.is_empty() {
                write!(f, [modifiers.format(), space()])?;
            }

            FormatMethodMember::from(node.clone()).fmt(f)
        });

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_inner,
                node.semicolon_token().as_ref()
            )]
        )
    }
}
