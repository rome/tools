use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsThrowStatement;
use rslint_parser::ast::JsThrowStatementFields;

impl ToFormatElement for JsThrowStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsThrowStatementFields {
            throw_token,
            argument,
            semicolon_token,
        } = self.as_fields();

        let throw_token = throw_token.format(formatter)?;
        let exception = argument.format(formatter)?;
        let semicolon = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(format_elements![
            throw_token,
            space_token(),
            exception,
            semicolon
        ])
    }
}
