use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsImportAssertion;

impl ToFormatElement for JsImportAssertion {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let assert_token = self.assert_token().format(formatter)?;
        let assertions = self.assertions().format(formatter)?;

        let result = formatter.format_delimited_soft_block_spaces(
            &self.l_curly_token()?,
            assertions,
            &self.r_curly_token()?,
        )?;

        Ok(format_elements![assert_token, space_token(), result])
    }
}
