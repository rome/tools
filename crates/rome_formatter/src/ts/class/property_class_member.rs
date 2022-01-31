use crate::{
    empty_element, format_elements, space_token, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::JsPropertyClassMember;

impl ToFormatElement for JsPropertyClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let static_token = if let Some(token) = self.static_token() {
            format_elements![formatter.format_token(&token)?, space_token()]
        } else {
            empty_element()
        };

        let init = if let Some(init) = self.value() {
            format_elements![space_token(), formatter.format_node(&init)?]
        } else {
            empty_element()
        };

        let semicolon = formatter
            .format_token(&self.semicolon_token())?
            .unwrap_or_else(|| token(';'));

        Ok(format_elements![
            static_token,
            formatter.format_node(&self.name()?)?,
            init,
            semicolon
        ])
    }
}
