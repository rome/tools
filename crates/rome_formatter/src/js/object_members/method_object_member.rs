use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsMethodObjectMember;

impl ToFormatElement for JsMethodObjectMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let async_token = self
            .async_token()
            .format_with_or_empty(formatter, |async_token| {
                format_elements![async_token, space_token()]
            })?;
        let star_token = self.star_token().format_or_empty(formatter)?;
        Ok(format_elements![
            async_token,
            star_token,
            self.name().format(formatter)?,
            // TODO self.type_params()
            self.parameters().format(formatter)?,
            // TODO self.return_type()
            space_token(),
            self.body().format(formatter)?,
        ])
    }
}
