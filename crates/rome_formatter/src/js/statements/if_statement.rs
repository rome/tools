use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, group_elements, soft_block_indent, space_token, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};

use rslint_parser::ast::{JsElseClause, JsIfStatement};

impl ToFormatElement for JsIfStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let else_caluse = self
            .else_clause()
            .format_with_or_empty(formatter, |else_clause| {
                format_elements![space_token(), else_clause]
            })?;

        Ok(format_elements![
            group_elements(format_elements![
                self.if_token().format(formatter)?,
                space_token(),
                group_elements(formatter.format_delimited(
                    &self.l_paren_token()?,
                    |open_token_trailing, close_token_leading| Ok(soft_block_indent(
                        format_elements![
                            open_token_trailing,
                            self.test().format(formatter)?,
                            close_token_leading
                        ]
                    )),
                    &self.r_paren_token()?,
                )?),
                space_token(),
            ]),
            self.consequent().format(formatter)?,
            else_caluse
        ])
    }
}
