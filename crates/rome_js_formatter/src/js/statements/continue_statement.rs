use crate::format_traits::FormatOptional;
use crate::utils::format_with_semicolon;
use crate::{formatted, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsContinueStatement;
use rome_js_syntax::JsContinueStatementFields;

impl FormatNode for JsContinueStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsContinueStatementFields {
            continue_token,
            label_token,
            semicolon_token,
        } = self.as_fields();

        let label = label_token.with_or_empty(|token| formatted![formatter, space_token(), token]);

        format_with_semicolon(
            formatter,
            formatted![formatter, continue_token.format(formatter)?, label]?,
            semicolon_token,
        )
    }
}
