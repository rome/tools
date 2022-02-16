use crate::{
    block_indent, format_elements, group_elements, if_group_breaks, join_elements,
    soft_block_indent, soft_line_break_or_space, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::{ast::TsTypeList, AstNode};
impl ToFormatElement for TsTypeList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![group_elements(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(self.clone(), || token(","))?,
        ))])
    }
}
