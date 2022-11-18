use crate::js::classes::property_class_member::{
    FormatClassPropertySemicolon, JsAnyPropertyClassMember,
};
use crate::prelude::*;
use crate::utils::JsAnyAssignmentLike;
use rome_formatter::write;
use rome_js_syntax::TsPropertySignatureClassMember;

#[derive(Debug, Clone, Default)]
pub struct FormatTsPropertySignatureClassMember;

impl FormatNodeRule<TsPropertySignatureClassMember> for FormatTsPropertySignatureClassMember {
    fn fmt_fields(
        &self,
        node: &TsPropertySignatureClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let semicolon_token = node.semicolon_token();

        write!(
            f,
            [
                JsAnyAssignmentLike::from(node.clone()),
                FormatClassPropertySemicolon::new(
                    &JsAnyPropertyClassMember::from(node.clone()),
                    semicolon_token.as_ref()
                )
            ]
        )
    }
}
