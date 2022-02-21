use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, hard_group_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
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

        Ok(hard_group_elements(format_elements![
            access_modifier
                .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?,
            name.format(formatter)?,
            parameters.format(formatter)?,
            space_token(),
            body.format(formatter)?
        ]))
    }
}
