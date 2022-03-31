use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    block_indent, format_elements, group_elements, if_group_breaks, if_group_fits_on_single_line,
    soft_block_indent, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::TsImplementsClause;
use rome_js_syntax::TsImplementsClauseFields;

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
