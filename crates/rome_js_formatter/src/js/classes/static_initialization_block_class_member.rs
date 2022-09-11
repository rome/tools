use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsStaticInitializationBlockClassMember;
use rome_js_syntax::JsStaticInitializationBlockClassMemberFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsStaticInitializationBlockClassMember;

impl FormatNodeRule<JsStaticInitializationBlockClassMember>
    for FormatJsStaticInitializationBlockClassMember
{
    fn fmt_fields(
        &self,
        node: &JsStaticInitializationBlockClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsStaticInitializationBlockClassMemberFields {
            static_token,
            l_curly_token,
            statements,
            r_curly_token,
        } = node.as_fields();

        write!(f, [static_token.format(), space(), l_curly_token.format()])?;

        if statements.is_empty() {
            write!(f, [format_dangling_comments(node.syntax()).indented()])?;
        } else {
            write!(f, [block_indent(&statements.format())])?;
        }

        write!(f, [r_curly_token.format()])
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsStaticInitializationBlockClassMember,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `fmt_fields
        Ok(())
    }
}
