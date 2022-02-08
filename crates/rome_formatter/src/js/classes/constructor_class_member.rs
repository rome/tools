use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsConstructorClassMember;

impl ToFormatElement for JsConstructorClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.name().format(formatter)?,
            self.parameters().format(formatter)?,
            space_token(),
            self.body().format(formatter)?
        ])
    }
}
