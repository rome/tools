use crate::formatter_traits::FormatOptionalTokenAndNode;
use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    concat_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsReturnStatement;

impl ToFormatElement for JsReturnStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let mut tokens = vec![self.return_token().format(formatter)?];

        if let Some(argument) = self.argument() {
            tokens.push(space_token());
            tokens.push(argument.format(formatter)?);
        }

        tokens.push(self.semicolon_token().format_or(formatter, || token(";"))?);

        Ok(concat_elements(tokens))
    }
}
