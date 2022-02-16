use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsPropertyParameter;

impl ToFormatElement for TsPropertyParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.accessibility().format(formatter)?,
            space_token(),
            self.formal_parameter().format(formatter)?
        ])
    }
}
