use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsPropertyClassMember;
use rslint_parser::ast::JsPropertyClassMemberFields;

impl ToFormatElement for JsPropertyClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPropertyClassMemberFields {
            declare_token,
            access_modifier,
            static_token,
            readonly_token,
            name,
            property_annotation,
            value,
            semicolon_token,
        } = self.as_fields();

        let declare_token = declare_token
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;
        let access_modifier = access_modifier
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;
        let static_token = static_token
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;
        let readonly_token = readonly_token
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;

        let property_annotation = property_annotation.format_or_empty(formatter)?;

        let init =
            value.format_with_or_empty(formatter, |node| format_elements![space_token(), node])?;

        let semicolon = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(format_elements![
            declare_token,
            access_modifier,
            static_token,
            readonly_token,
            name.format(formatter)?,
            property_annotation,
            init,
            semicolon
        ])
    }
}
