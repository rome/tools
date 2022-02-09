use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsThrowStatement;

impl ToFormatElement for JsThrowStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let throw_token = self.throw_token().format(formatter)?;
        let exception = self.argument().format(formatter)?;
        let semicolon = self.semicolon_token().format_or(formatter, || token(";"))?;

        Ok(format_elements![
            throw_token,
            space_token(),
            exception,
            semicolon
        ])
    }
}
