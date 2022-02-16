use crate::formatter_traits::FormatTokenAndNode;

use crate::FormatResult;

use crate::{
    format_elements, join_elements_hard_line, space_token, FormatElement, Formatter,
    ToFormatElement,
};

use rslint_parser::ast::JsSwitchStatement;

impl ToFormatElement for JsSwitchStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.switch_token().format(formatter)?,
            space_token(),
            formatter.format_delimited_soft_block_indent(
                &self.l_paren_token()?,
                self.discriminant().format(formatter)?,
                &self.r_paren_token()?,
            )?,
            space_token(),
            formatter.format_delimited_block_indent(
                &self.l_curly_token()?,
                join_elements_hard_line(
                    self.cases()
                        .into_iter()
                        .zip(formatter.format_nodes(self.cases())?)
                ),
                &self.r_curly_token()?
            )?
        ])
    }
}
