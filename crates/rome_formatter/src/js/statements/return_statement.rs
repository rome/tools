use crate::formatter_traits::FormatOptionalTokenAndNode;
use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsReturnStatement;

impl ToFormatElement for JsReturnStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let return_token = self.return_token().format(formatter)?;

        let argument = self
            .argument()
            .format_with_or_empty(formatter, |argument| {
                format_elements![space_token(), argument]
            })?;

        let semicolon = self.semicolon_token().format_or(formatter, || token(";"))?;

        Ok(format_elements![return_token, argument, semicolon])
    }
}
