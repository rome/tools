use crate::prelude::*;
use crate::utils::{FormatWithSemicolon, JsAnyAssignmentLike};
use rome_formatter::{format_args, write};
use rome_js_syntax::TsTypeAliasDeclaration;

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeAliasDeclaration;

impl FormatNodeRule<TsTypeAliasDeclaration> for FormatTsTypeAliasDeclaration {
    fn fmt_fields(&self, node: &TsTypeAliasDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let type_token = node.type_token()?;
        let semicolon = node.semicolon_token();
        let assignment_like = format_with(|f| write!(f, [JsAnyAssignmentLike::from(node.clone())]));
        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args![type_token.format(), space(), group(&assignment_like)],
                semicolon.as_ref()
            )]
        )
    }
}
