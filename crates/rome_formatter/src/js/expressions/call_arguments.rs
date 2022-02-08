use crate::{
    format_elements, group_elements, join_elements, soft_block_indent, soft_line_break_or_space,
    token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsCallArguments;

impl ToFormatElement for JsCallArguments {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let args_tokens = formatter.format_separated(self.args(), || token(","))?;

        Ok(group_elements(formatter.format_delimited(
            &self.l_paren_token()?,
            |open_token_trailing, close_token_leading| {
                Ok(soft_block_indent(format_elements![
                    open_token_trailing,
                    join_elements(soft_line_break_or_space(), args_tokens),
                    close_token_leading
                ]))
            },
            &self.r_paren_token()?,
        )?))
    }
}
