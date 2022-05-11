use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsNamedImportSpecifiers;
use rome_js_syntax::JsNamedImportSpecifiersFields;

impl FormatNodeFields<JsNamedImportSpecifiers> for FormatNodeRule<JsNamedImportSpecifiers> {
    fn format_fields(
        node: &JsNamedImportSpecifiers,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsNamedImportSpecifiersFields {
            l_curly_token,
            specifiers,
            r_curly_token,
        } = node.as_fields();

        formatter.format_delimited_soft_block_spaces(
            &l_curly_token?,
            formatted![formatter, [specifiers.format()]]?,
            &r_curly_token?,
        )
    }
}
