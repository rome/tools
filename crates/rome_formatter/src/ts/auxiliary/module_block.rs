use crate::formatter_traits::FormatTokenAndNode;
use crate::{join_elements_hard_line, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsModuleBlockFields;
use rslint_parser::{ast::TsModuleBlock, AstNode};

impl ToFormatElement for TsModuleBlock {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsModuleBlockFields {
            l_curly_token,
            items,
            r_curly_token,
        } = self.as_fields();

        formatter.format_delimited_block_indent(
            &l_curly_token?,
            items.format(formatter)?,
            &r_curly_token?,
        )
    }
}
