use crate::formatter::TryFormatNodeListExtension;
use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsStaticInitializationBlockClassMember;
use rome_js_syntax::JsStaticInitializationBlockClassMemberFields;

impl FormatNodeFields<JsStaticInitializationBlockClassMember>
    for FormatNodeRule<JsStaticInitializationBlockClassMember>
{
    fn format_fields(
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

        let format_statements = format_with(|f| {
            f.join_with(&hard_line_break())
                .entries(statements.try_format_nodes())
                .finish()
        });

        write!(
            f,
            [
                f.delimited(&l_curly_token?, &format_statements, &r_curly_token?)
                    .block_indent()
            ]
        )
    }
}
