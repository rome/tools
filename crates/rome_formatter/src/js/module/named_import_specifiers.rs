use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsNamedImportSpecifiers;
use rome_js_syntax::JsNamedImportSpecifiersFields;

impl ToFormatElement for JsNamedImportSpecifiers {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsNamedImportSpecifiersFields {
            l_curly_token,
            specifiers,
            r_curly_token,
        } = self.as_fields();

        let specifiers = specifiers.format(formatter)?;

        formatter.format_delimited_soft_block_spaces(&l_curly_token?, specifiers, &r_curly_token?)
    }
}
