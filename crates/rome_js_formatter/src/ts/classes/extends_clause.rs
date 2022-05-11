use crate::prelude::*;
use rome_js_syntax::{TsExtendsClause, TsExtendsClauseFields};

impl FormatNode for TsExtendsClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsExtendsClauseFields {
            extends_token,
            types,
        } = self.as_fields();
        let extends = extends_token.format(formatter)?;
        let types = types.format(formatter)?;

        Ok(group_elements(formatted![
            formatter,
            if_group_breaks(block_indent(formatted![
                formatter,
                extends.clone(),
                space_token(),
                soft_block_indent(types.clone())
            ]?)),
            if_group_fits_on_single_line(formatted![formatter, extends, space_token(), types]?),
        ]?))
    }
}
