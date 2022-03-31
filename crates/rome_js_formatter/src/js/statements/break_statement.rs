use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::utils::format_with_semicolon;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsBreakStatement;
use rome_js_syntax::JsBreakStatementFields;

impl ToFormatElement for JsBreakStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsBreakStatementFields {
            break_token,
            label_token,
            semicolon_token,
        } = self.as_fields();

        let label = label_token
            .format_with_or_empty(formatter, |label| format_elements![space_token(), label])?;

        format_with_semicolon(
            formatter,
            format_elements![break_token.format(formatter)?, label],
            semicolon_token,
        )
    }
}
