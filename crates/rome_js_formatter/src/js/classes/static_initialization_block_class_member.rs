use crate::prelude::*;

use rome_js_syntax::JsStaticInitializationBlockClassMember;
use rome_js_syntax::JsStaticInitializationBlockClassMemberFields;

impl FormatNode for JsStaticInitializationBlockClassMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsStaticInitializationBlockClassMemberFields {
            static_token,
            l_curly_token,
            statements,
            r_curly_token,
        } = self.as_fields();

        let static_token = static_token.format(formatter)?;
        let separated = formatter.format_delimited_block_indent(
            &l_curly_token?,
            formatter.format_list(statements),
            &r_curly_token?,
        )?;
        formatted![formatter, static_token, space_token(), separated]
    }
}
