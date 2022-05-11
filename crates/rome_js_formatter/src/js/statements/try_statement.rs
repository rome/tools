use crate::prelude::*;

use rome_js_syntax::JsTryStatement;
use rome_js_syntax::JsTryStatementFields;

impl FormatNode for JsTryStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsTryStatementFields {
            try_token,
            body,
            catch_clause,
        } = self.as_fields();

        Ok(hard_group_elements(formatted![
            formatter,
            try_token.format(formatter)?,
            space_token(),
            body.format(formatter)?,
            space_token(),
            catch_clause.format(formatter)?,
        ]?))
    }
}
