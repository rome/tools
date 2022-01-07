use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsThrowStatement;

impl ToFormatElement for JsThrowStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let throw_token = formatter.format_token(&self.throw_token()?)?;
        let exception = formatter.format_node(self.argument()?)?;

        let semicolon = formatter.format_or_create_token(self.semicolon_token(), || token(';'))?;

        Ok(format_elements![
            throw_token,
            space_token(),
            exception,
            semicolon
        ])
    }
}
