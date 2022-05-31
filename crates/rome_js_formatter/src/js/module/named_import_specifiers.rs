use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsNamedImportSpecifiers;
use rome_js_syntax::JsNamedImportSpecifiersFields;

impl FormatNodeFields<JsNamedImportSpecifiers> for FormatNodeRule<JsNamedImportSpecifiers> {
    fn format_fields(
        node: &JsNamedImportSpecifiers,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsNamedImportSpecifiersFields {
            l_curly_token,
            specifiers,
            r_curly_token,
        } = node.as_fields();

        formatter
            .delimited(
                &l_curly_token?,
                formatted![formatter, [specifiers.format()]]?,
                &r_curly_token?,
            )
            .soft_block_spaces()
            .finish()
    }
}
