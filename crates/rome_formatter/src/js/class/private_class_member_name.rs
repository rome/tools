use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsPrivateClassMemberName;

impl ToFormatElement for JsPrivateClassMemberName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.hash_token().format(formatter)?,
            self.id_token().format(formatter)?,
        ])
    }
}
