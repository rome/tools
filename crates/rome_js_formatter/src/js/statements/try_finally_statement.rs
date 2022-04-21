use crate::format_traits::FormatOptional;

use crate::{
    format_elements, hard_group_elements, space_token, Format, FormatElement, FormatNode,
    FormatResult, Formatter,
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
            .format_with_or_empty(formatter, |catch_clause| {
                format_elements![space_token(), catch_clause]
            })?;

        Ok(hard_group_elements(format_elements![
            try_token.format(formatter)?,
            space_token(),
            body.format(formatter)?,
            formatted_catch_clause,
            space_token(),
            finally_clause.format(formatter)?
        ]))
    }
}
