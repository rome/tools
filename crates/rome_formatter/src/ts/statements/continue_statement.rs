use crate::{
    empty_element, format_elements, space_token, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::JsContinueStatement;

impl ToFormatElement for JsContinueStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let label = if let Some(label_token) = self.label_token() {
            format_elements![space_token(), formatter.format_token(&label_token)?]
        } else {
            empty_element()
        };

        let semicolon = formatter
            .format_token(&self.semicolon_token())?
            .unwrap_or_else(|| token(";"));

        Ok(format_elements![
            formatter.format_token(&self.continue_token()?)?,
            label,
            semicolon
        ])
    }
}
