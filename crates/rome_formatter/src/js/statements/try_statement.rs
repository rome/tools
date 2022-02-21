use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, hard_group_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rslint_parser::ast::JsTryStatement;
use rslint_parser::ast::JsTryStatementFields;

impl ToFormatElement for JsTryStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsTryStatementFields {
            try_token,
            body,
            catch_clause,
        } = self.as_fields();

        Ok(hard_group_elements(format_elements![
            try_token.format(formatter)?,
            space_token(),
            body.format(formatter)?,
            space_token(),
            catch_clause.format(formatter)?,
        ]))
    }
}
