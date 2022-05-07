use crate::format_traits::FormatOptional;
use crate::utils::format_with_semicolon;
use crate::{formatted, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsBreakStatement;
use rome_js_syntax::JsBreakStatementFields;

impl FormatNode for JsBreakStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsBreakStatementFields {
            break_token,
            label_token,
            semicolon_token,
        } = self.as_fields();

        let label = label_token.with_or_empty(|label| formatted![formatter, space_token(), label]);

        format_with_semicolon(
            formatter,
            formatted![formatter, break_token.format(formatter)?, label]?,
            semicolon_token,
        )
    }
}
