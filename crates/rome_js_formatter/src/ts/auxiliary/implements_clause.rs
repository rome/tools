use crate::{
    block_indent, group_elements, if_group_breaks, if_group_fits_on_single_line,
    soft_block_indent, space_token, Format, FormatElement, FormatNode, Formatter,
};
use rome_formatter::FormatResult;
use rome_js_syntax::TsImplementsClause;
use rome_js_syntax::TsImplementsClauseFields;

impl FormatNode for TsImplementsClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsImplementsClauseFields {
            implements_token,
            types,
        } = self.as_fields();

        let implements_token = implements_token.format(formatter)?;
        let types = types.format(formatter)?;

        Ok(group_elements(formatted![
            formatter,
            if_group_breaks(block_indent(formatted![
                formatter,
                implements_token.clone(),
                space_token(),
                soft_block_indent(types.clone())
            ]?)),
            if_group_fits_on_single_line(formatted![
                formatter,
                implements_token,
                space_token(),
                types
            ]?),
        ]?))
    }
}
