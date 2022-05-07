use crate::format_traits::FormatOptional;
use rome_formatter::FormatResult;

use crate::{
    formatted, hard_group_elements, space_token, Format, FormatElement, FormatNode, Formatter,
};

use rome_js_syntax::JsTryFinallyStatement;
use rome_js_syntax::JsTryFinallyStatementFields;

impl FormatNode for JsTryFinallyStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsTryFinallyStatementFields {
            try_token,
            body,
            catch_clause,
            finally_clause,
        } = self.as_fields();

        let formatted_catch_clause = catch_clause
            .with_or_empty(|catch_clause| formatted![formatter, space_token(), catch_clause]);

        Ok(hard_group_elements(formatted![
            formatter,
            try_token.format(formatter)?,
            space_token(),
            body.format(formatter)?,
            formatted_catch_clause,
            space_token(),
            finally_clause.format(formatter)?
        ]?))
    }
}
