use crate::{
    format_elements, group_elements, soft_block_indent, soft_line_break_or_space, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsObjectType;

impl ToFormatElement for TsObjectType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(group_elements(formatter.format_delimited(
            &self.l_curly_token()?,
            |open_token_trailing, close_token_leading| {
                let list = self.members().to_format_element(formatter)?;
                Ok(format_elements![
                    soft_block_indent(format_elements![
                        open_token_trailing,
                        list,
                        close_token_leading
                    ]),
                    soft_line_break_or_space()
                ])
            },
            &self.r_curly_token()?,
        )?))
    }
}
