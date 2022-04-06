use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    block_indent, format_elements, group_elements, if_group_breaks, if_group_fits_on_single_line,
    soft_block_indent, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::{TsExtendsClause, TsExtendsClauseFields};

impl ToFormatElement for TsExtendsClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsExtendsClauseFields {
            extends_token,
            types,
        } = self.as_fields();
        let extends = extends_token.format(formatter)?;
        let types = types.format(formatter)?;

        Ok(group_elements(format_elements![
            if_group_breaks(block_indent(format_elements![
                extends.clone(),
                space_token(),
                soft_block_indent(types.clone())
            ])),
            if_group_fits_on_single_line(format_elements![extends, space_token(), types]),
        ]))
    }
}
