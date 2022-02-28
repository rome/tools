use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement, group_elements, if_group_breaks, block_indent, soft_block_indent, if_group_fits_on_single_line,
};
use rslint_parser::ast::TsImplementsClause;
use rslint_parser::ast::TsImplementsClauseFields;

impl ToFormatElement for TsImplementsClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsImplementsClauseFields {
            implements_token,
            types,
        } = self.as_fields();

        let implements_token = implements_token.format(formatter)?;
        let types = types.format(formatter)?;

        Ok(group_elements(format_elements![
            if_group_breaks(block_indent(format_elements![
                implements_token.clone(),
                space_token(),
                soft_block_indent(types.clone())
            ])),
            if_group_fits_on_single_line(format_elements![implements_token, space_token(), types]),
        ]))
    }
}
