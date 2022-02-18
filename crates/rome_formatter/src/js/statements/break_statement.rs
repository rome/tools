use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsBreakStatement;
use rslint_parser::ast::JsBreakStatementFields;

impl ToFormatElement for JsBreakStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsBreakStatementFields {
            break_token,
            label_token,
            semicolon_token,
        } = self.as_fields();

        let label = label_token
            .format_with_or_empty(formatter, |label| format_elements![space_token(), label])?;

        let semicolon = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(format_elements![
            break_token.format(formatter)?,
            label,
            semicolon,
        ])
    }
}
