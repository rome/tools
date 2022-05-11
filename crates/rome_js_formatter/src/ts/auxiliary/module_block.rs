use crate::prelude::*;
use rome_js_syntax::TsModuleBlock;
use rome_js_syntax::TsModuleBlockFields;

impl FormatNode for TsModuleBlock {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
