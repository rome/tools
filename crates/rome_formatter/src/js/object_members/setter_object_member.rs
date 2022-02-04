use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsSetterObjectMember;

impl ToFormatElement for JsSetterObjectMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.set_token().format(formatter)?,
            space_token(),
            self.name().format(formatter)?,
            self.l_paren_token().format(formatter)?,
            self.parameter().format(formatter)?,
            self.r_paren_token().format(formatter)?,
            space_token(),
            self.body().format(formatter)?,
        ])
    }
}
