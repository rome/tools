use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsNamedImportSpecifiers;
use rome_js_syntax::JsNamedImportSpecifiersFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsNamedImportSpecifiers;

impl FormatNodeRule<JsNamedImportSpecifiers> for FormatJsNamedImportSpecifiers {
    fn fmt_fields(&self, node: &JsNamedImportSpecifiers, f: &mut JsFormatter) -> FormatResult<()> {
        let JsNamedImportSpecifiersFields {
            l_curly_token,
            specifiers,
            r_curly_token,
        } = node.as_fields();

        if specifiers.is_empty() {
            write!(f, [l_curly_token.format(), r_curly_token.format()])
        } else {
            write!(
                f,
                [
                    format_delimited(&l_curly_token?, &specifiers.format(), &r_curly_token?,)
                        .soft_block_spaces()
                ]
            )
        }
    }
}
