use crate::prelude::*;

use rome_js_syntax::JsNamedImportSpecifiers;
use rome_js_syntax::JsNamedImportSpecifiersFields;

impl FormatNode for JsNamedImportSpecifiers {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsNamedImportSpecifiersFields {
            l_curly_token,
            specifiers,
            r_curly_token,
        } = self.as_fields();

        let specifiers = specifiers.format(formatter)?;

        formatter.format_delimited_soft_block_spaces(&l_curly_token?, specifiers, &r_curly_token?)
    }
}
