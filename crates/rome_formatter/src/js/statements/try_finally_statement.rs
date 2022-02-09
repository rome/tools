use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsTryFinallyStatement;

impl ToFormatElement for JsTryFinallyStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let formatted_catch_clause = self
            .catch_clause()
            .format_with_or_empty(formatter, |catch_clause| {
                format_elements![space_token(), catch_clause]
            })?;

        Ok(format_elements![
            self.try_token().format(formatter)?,
            space_token(),
            self.body().format(formatter)?,
            formatted_catch_clause,
            space_token(),
            self.finally_clause().format(formatter)?
        ])
    }
}
