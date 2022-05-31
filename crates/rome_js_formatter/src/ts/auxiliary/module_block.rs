use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsModuleBlock;
use rome_js_syntax::TsModuleBlockFields;

impl FormatNodeFields<TsModuleBlock> for FormatNodeRule<TsModuleBlock> {
    fn format_fields(node: &TsModuleBlock, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let TsModuleBlockFields {
            l_curly_token,
            items,
            r_curly_token,
        } = node.as_fields();

        formatter
            .delimited(
                &l_curly_token?,
                formatted![formatter, [items.format()]]?,
                &r_curly_token?,
            )
            .block_indent()
            .finish()
    }
}
