use crate::formatter_traits::FormatOptionalTokenAndNode;

use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsReturnStatement;
use rslint_parser::ast::JsReturnStatementFields;

impl ToFormatElement for JsReturnStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsReturnStatementFields {
            return_token,
            argument,
            semicolon_token,
        } = self.as_fields();

        let return_token = return_token.format(formatter)?;

        let argument = argument.format_with_or_empty(formatter, |argument| {
            format_elements![space_token(), argument]
        })?;

        let semicolon = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(format_elements![return_token, argument, semicolon])
    }
}
