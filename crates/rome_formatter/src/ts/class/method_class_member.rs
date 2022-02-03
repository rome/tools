use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsMethodClassMember;

impl ToFormatElement for JsMethodClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let async_token = self
            .async_token()
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;
        let static_token = self
            .static_token()
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;
        let star_token = self.star_token().format_or_empty(formatter)?;
        let name = self.name().format(formatter)?;
        let params = self.parameters().format(formatter)?;
        let body = self.body().format(formatter)?;
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
