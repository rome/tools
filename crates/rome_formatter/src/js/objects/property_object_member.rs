use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsPropertyObjectMember;

impl ToFormatElement for JsPropertyObjectMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let key = self.name().format(formatter)?;
        let colon = self.colon_token().format(formatter)?;
        let value = self.value().format(formatter)?;
        Ok(format_elements![key, colon, space_token(), value])
    }
}
