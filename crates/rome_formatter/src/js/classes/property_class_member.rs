use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsPropertyClassMember;

impl ToFormatElement for JsPropertyClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let static_token = self
            .static_token()
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;

        let init = self
            .value()
            .format_with_or_empty(formatter, |node| format_elements![space_token(), node])?;

        let semicolon = self.semicolon_token().format_or(formatter, || token(";"))?;

        Ok(format_elements![
            static_token,
            self.name().format(formatter)?,
            init,
            semicolon
        ])
    }
}
