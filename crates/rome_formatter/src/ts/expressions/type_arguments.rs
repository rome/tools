use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, group_elements, if_group_breaks, soft_block_indent, soft_line_break,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsTypeArguments;

impl ToFormatElement for TsTypeArguments {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(group_elements(formatter.format_delimited(
            &self.l_angle_token()?,
            |open_token_trailing, close_token_leading| {
                Ok(format_elements![
                    if_group_breaks(soft_line_break()),
                    soft_block_indent(format_elements![
                        open_token_trailing,
                        self.ts_type_argument_list().format(formatter)?,
                        close_token_leading,
                    ]),
                    if_group_breaks(soft_line_break()),
                ])
            },
            &self.r_angle_token()?,
        )?))
    }
}
