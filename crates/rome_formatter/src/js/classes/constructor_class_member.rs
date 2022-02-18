use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsConstructorClassMember;
use rslint_parser::ast::JsConstructorClassMemberFields;

impl ToFormatElement for JsConstructorClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsConstructorClassMemberFields {
            access_modifier,
            name,
            parameters,
            body,
        } = self.as_fields();

        Ok(format_elements![
            name.format(formatter)?,
            parameters.format(formatter)?,
            space_token(),
            body.format(formatter)?
        ])
    }
}
