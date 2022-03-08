use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsContinueStatement;
use rome_js_syntax::JsContinueStatementFields;

impl ToFormatElement for JsContinueStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsContinueStatementFields {
            continue_token,
            label_token,
            semicolon_token,
        } = self.as_fields();

        let label = label_token
            .format_with_or_empty(formatter, |token| format_elements![space_token(), token])?;

        let semicolon = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(format_elements![
            continue_token.format(formatter)?,
            label,
            semicolon
        ])
    }
}
