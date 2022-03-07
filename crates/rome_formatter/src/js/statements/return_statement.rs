use crate::formatter_traits::FormatOptionalTokenAndNode;
use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    empty_element, format_elements, space_token, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rome_js_syntax::JsReturnStatement;
use rome_js_syntax::JsReturnStatementFields;
use rslint_parser::AstNode;
use rslint_parser::JsSyntaxKind;

impl ToFormatElement for JsReturnStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsReturnStatementFields {
            return_token,
            argument,
            semicolon_token,
        } = self.as_fields();

        let return_token = return_token.format(formatter)?;

        let argument = if let Some(argument) = argument {
            if matches!(
                argument.syntax().kind(),
                JsSyntaxKind::JS_SEQUENCE_EXPRESSION
            ) {
                format_elements![
                    space_token(),
                    token("("),
                    argument.format(formatter)?,
                    token(")")
                ]
            } else {
                format_elements![space_token(), argument.format(formatter)?]
            }
        } else {
            empty_element()
        };

        let semicolon = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(format_elements![return_token, argument, semicolon])
    }
}
