use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, group_elements, if_group_breaks, if_group_fits_on_single_line,
    soft_block_indent, soft_line_break, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::TsExtendsClause;

impl ToFormatElement for TsExtendsClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let extends = self.extends_token().format(formatter)?;
        let types = self.types().format(formatter)?;

        Ok(format_elements![group_elements(format_elements![
            if_group_breaks(soft_block_indent(format_elements![
                soft_line_break(),
                extends.clone(),
                space_token(),
                soft_block_indent(types.clone())
            ])),
            if_group_fits_on_single_line(format_elements![extends, space_token(), types]),
        ])])
    }
}
