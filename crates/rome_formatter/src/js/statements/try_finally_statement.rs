use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, hard_group_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rome_js_syntax::JsTryFinallyStatement;
use rome_js_syntax::JsTryFinallyStatementFields;

impl ToFormatElement for JsTryFinallyStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
