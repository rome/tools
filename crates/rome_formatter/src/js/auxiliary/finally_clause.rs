use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, group_elements, soft_block_indent, space_token, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};

use rslint_parser::ast::{
    JsCatchClause, JsCatchDeclaration, JsFinallyClause, JsTryFinallyStatement, JsTryStatement,
};

impl ToFormatElement for JsFinallyClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.finally_token().format(formatter)?,
            space_token(),
            self.body().format(formatter)?
        ])
    }
}
