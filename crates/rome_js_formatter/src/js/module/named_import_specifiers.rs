use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsNamedImportSpecifiers;
use rome_js_syntax::JsNamedImportSpecifiersFields;

impl FormatNodeFields<JsNamedImportSpecifiers> for FormatNodeRule<JsNamedImportSpecifiers> {
    fn fmt_fields(node: &JsNamedImportSpecifiers, f: &mut JsFormatter) -> FormatResult<()> {
        let JsNamedImportSpecifiersFields {
            l_curly_token,
            specifiers,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_delimited(&l_curly_token?, &specifiers.format(), &r_curly_token?,)
                    .soft_block_spaces()
            ]
        )
    }
}
