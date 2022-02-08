use crate::formatter_traits::FormatTokenAndNode;

use crate::{block_indent, FormatResult};

use crate::{
    format_element::indent, format_elements, group_elements, hard_line_break,
    join_elements_hard_line, soft_block_indent, space_token, FormatElement, Formatter,
    ToFormatElement,
};

use rslint_parser::ast::{JsAnySwitchClause, JsCaseClause, JsDefaultClause, JsSwitchStatement};

impl ToFormatElement for JsSwitchStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.switch_token().format(formatter)?,
            space_token(),
            group_elements(formatter.format_delimited(
                &self.l_paren_token()?,
                |open_token_trailing, close_token_leading| Ok(soft_block_indent(format_elements![
                    open_token_trailing,
                    self.discriminant().format(formatter)?,
                    close_token_leading,
                ])),
                &self.r_paren_token()?,
            )?),
            space_token(),
            group_elements(formatter.format_delimited(
                &self.l_curly_token()?,
                |open_token_trailing, close_token_leading| {
                    Ok(block_indent(format_elements![
                        open_token_trailing,
                        join_elements_hard_line(
                            self.cases()
                                .into_iter()
                                .zip(formatter.format_nodes(self.cases())?)
                        ),
                        close_token_leading,
                    ]))
                },
                &self.r_curly_token()?
            )?)
        ])
    }
}
