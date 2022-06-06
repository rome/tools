use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsStaticInitializationBlockClassMember;
use rome_js_syntax::JsStaticInitializationBlockClassMemberFields;

impl FormatNodeFields<JsStaticInitializationBlockClassMember>
    for FormatNodeRule<JsStaticInitializationBlockClassMember>
{
    fn fmt_fields(
        node: &JsStaticInitializationBlockClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsStaticInitializationBlockClassMemberFields {
            static_token,
            l_curly_token,
            statements,
            r_curly_token,
        } = node.as_fields();

        write!(f, [static_token.format(), space_token()])?;

        write!(
            f,
            [
                format_delimited(&l_curly_token?, &statements.format(), &r_curly_token?)
                    .block_indent()
            ]
        )
    }
}
