use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, group_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rslint_parser::ast::JsIfStatement;
use rslint_parser::ast::JsIfStatementFields;

impl ToFormatElement for JsIfStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsIfStatementFields {
            if_token,
            l_paren_token,
            test,
            r_paren_token,
            consequent,
            else_clause,
        } = self.as_fields();

        let else_caluse = else_clause.format_with_or_empty(formatter, |else_clause| {
            format_elements![space_token(), else_clause]
        })?;

        Ok(format_elements![
            group_elements(format_elements![
                if_token.format(formatter)?,
                space_token(),
                formatter.format_delimited_soft_block_indent(
                    &l_paren_token?,
                    test.format(formatter)?,
                    &r_paren_token?,
                )?,
                space_token(),
            ]),
            consequent.format(formatter)?,
            else_caluse
        ])
    }
}
