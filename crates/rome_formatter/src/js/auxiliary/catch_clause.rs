use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, group_elements, soft_block_indent, space_token, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};

use rslint_parser::ast::{
    JsCatchClause, JsCatchDeclaration, JsFinallyClause, JsTryFinallyStatement, JsTryStatement,
};

impl ToFormatElement for JsCatchClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.declaration().format_with_or(
            formatter,
            |declaration| {
                Ok(format_elements![
                    self.catch_token().format(formatter)?,
                    space_token(),
                    declaration,
                    space_token(),
                    self.body().format(formatter)?
                ])
            },
            || {
                Ok(format_elements![
                    self.catch_token().format(formatter)?,
                    space_token(),
                    self.body().format(formatter)?
                ])
            },
        )
    }
}
