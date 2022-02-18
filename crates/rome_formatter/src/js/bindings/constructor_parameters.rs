use crate::{
    formatter_traits::FormatTokenAndNode, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsConstructorParameters;
use rslint_parser::ast::JsConstructorParametersFields;

impl ToFormatElement for JsConstructorParameters {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsConstructorParametersFields {
            l_paren_token,
            parameters,
            r_paren_token,
        } = self.as_fields();

        formatter.format_delimited_soft_block_indent(
            &l_paren_token?,
            parameters.format(formatter)?,
            &r_paren_token?,
        )
    }
}
