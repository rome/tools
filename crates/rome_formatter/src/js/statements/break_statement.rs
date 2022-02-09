use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsBreakStatement;

impl ToFormatElement for JsBreakStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let label = self
            .label_token()
            .format_with_or_empty(formatter, |label| format_elements![space_token(), label])?;

        let semicolon = self.semicolon_token().format_or(formatter, || token(";"))?;

        Ok(format_elements![
            self.break_token().format(formatter)?,
            label,
            semicolon,
        ])
    }
}
