use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsNamedImportSpecifiers;

impl ToFormatElement for JsNamedImportSpecifiers {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let specifiers = self.specifiers().format(formatter)?;

        formatter.format_delimited_soft_block_spaces(
            &self.l_curly_token()?,
            specifiers,
            &self.r_curly_token()?,
        )
    }
}
