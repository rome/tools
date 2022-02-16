use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsReadonlyPropertyParameter;

impl ToFormatElement for TsReadonlyPropertyParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let accessibility = self
            .accessibility()
            .format_with_or_empty(formatter, |accessibility| {
                format_elements![accessibility, space_token()]
            })?;

        Ok(format_elements![
            accessibility,
            self.readonly_token().format(formatter)?,
            space_token(),
            self.formal_parameter().format(formatter)?,
        ])
    }
}
