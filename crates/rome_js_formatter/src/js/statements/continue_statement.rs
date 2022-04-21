use crate::format_traits::FormatOptional;
use crate::utils::format_with_semicolon;
use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};

use rome_js_syntax::JsContinueStatement;
use rome_js_syntax::JsContinueStatementFields;

impl FormatNode for JsContinueStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsContinueStatementFields {
            continue_token,
            label_token,
            semicolon_token,
        } = self.as_fields();

        let label = label_token
            .format_with_or_empty(formatter, |token| format_elements![space_token(), token])?;

        format_with_semicolon(
            formatter,
            format_elements![continue_token.format(formatter)?, label],
            semicolon_token,
        )
    }
}
