use crate::{format_elements, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsFinallyClause;
use rome_js_syntax::JsFinallyClauseFields;

impl FormatNode for JsFinallyClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsFinallyClauseFields {
            finally_token,
            body,
        } = self.as_fields();

        Ok(format_elements![
            finally_token.format(formatter)?,
            space_token(),
            body.format(formatter)?
        ])
    }
}
