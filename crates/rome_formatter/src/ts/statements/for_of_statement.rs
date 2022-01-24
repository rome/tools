use rslint_parser::ast::JsForOfStatement;

use crate::{
    format_elements, group_elements, soft_block_indent, soft_line_break_or_space, space_token,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};

impl ToFormatElement for JsForOfStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let for_token = formatter.format_token(&self.for_token()?)?;
        let initializer = formatter.format_node(&self.initializer()?)?;
        let of_token = formatter.format_token(&self.of_token()?)?;
        let expression = formatter.format_node(&self.expression()?)?;
        let body = formatter.format_node(&self.body()?)?;

        Ok(format_elements![
            for_token,
            space_token(),
            formatter.format_delimited(
                &self.l_paren_token()?,
                |open_token_trailing, close_token_leading| Ok(group_elements(soft_block_indent(
                    format_elements![
                        open_token_trailing,
                        initializer,
                        soft_line_break_or_space(),
                        of_token,
                        soft_line_break_or_space(),
                        expression,
                        close_token_leading,
                    ]
                ))),
                &self.r_paren_token()?
            )?,
            space_token(),
            body
        ])
    }
}
