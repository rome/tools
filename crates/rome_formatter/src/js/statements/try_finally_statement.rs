use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, group_elements, soft_block_indent, space_token, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};

use rslint_parser::ast::{
    JsCatchClause, JsCatchDeclaration, JsFinallyClause, JsTryFinallyStatement, JsTryStatement,
};

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
