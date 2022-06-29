use crate::prelude::*;
use crate::utils::{FormatWithSemicolon, JsAnyAssignmentLike};
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
        let body = format_with(|f| write!(f, [JsAnyAssignmentLike::from(node.clone())]));
        write!(
            f,
            [FormatWithSemicolon::new(&body, semicolon_token.as_ref())]
        )
    }
}
