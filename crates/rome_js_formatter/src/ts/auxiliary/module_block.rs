use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsModuleBlock;
use rome_js_syntax::TsModuleBlockFields;

impl FormatNodeFields<TsModuleBlock> for FormatNodeRule<TsModuleBlock> {
    fn format_fields(node: &TsModuleBlock, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsModuleBlockFields {
            l_curly_token,
            items,
            r_curly_token,
        } = node.as_fields();

        formatter.format_delimited_block_indent(
            &l_curly_token?,
            formatted![formatter, items.format()]?,
            &r_curly_token?,
        )
    }
}
