use crate::prelude::*;
use crate::utils::FormatTypeMemberSeparator;

use crate::js::classes::method_class_member::FormatJsAnyMethodMember;
use rome_formatter::write;
use rome_js_syntax::TsMethodSignatureTypeMember;

#[derive(Debug, Clone, Default)]
pub struct FormatTsMethodSignatureTypeMember;

impl FormatNodeRule<TsMethodSignatureTypeMember> for FormatTsMethodSignatureTypeMember {
    fn fmt_fields(
        &self,
        node: &TsMethodSignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        write![
            f,
            [
                FormatJsAnyMethodMember::from(node.clone()),
                FormatTypeMemberSeparator::new(node.separator_token().as_ref())
            ]
        ]
    }
}
