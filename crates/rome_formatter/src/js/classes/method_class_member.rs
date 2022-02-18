use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsMethodClassMember;
use rslint_parser::ast::JsMethodClassMemberFields;

impl ToFormatElement for JsMethodClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsMethodClassMemberFields {
            access_modifier,
            static_token,
            abstract_token,
            async_token,
            star_token,
            name,
            question_mark_token,
            type_parameters,
            parameters,
            return_type_annotation,
            body,
        } = self.as_fields();

        let async_token = async_token
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;
        let static_token = static_token
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;
        let star_token = star_token.format_or_empty(formatter)?;
        let name = name.format(formatter)?;
        let params = parameters.format(formatter)?;
        let body = body.format(formatter)?;
        Ok(format_elements![
            static_token,
            async_token,
            star_token,
            name,
            params,
            space_token(),
            body
        ])
    }
}
