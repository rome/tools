use crate::prelude::*;

use crate::builders::format_delimited;
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

        write!(f, [static_token.format(), space()])?;

        write!(
            f,
            [
                format_delimited(&l_curly_token?, &statements.format(), &r_curly_token?)
                    .block_indent()
            ]
        )
    }
}
