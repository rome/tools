use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsModuleBlock;
use rome_js_syntax::TsModuleBlockFields;

impl FormatNodeFields<TsModuleBlock> for FormatNodeRule<TsModuleBlock> {
    fn fmt_fields(node: &TsModuleBlock, f: &mut JsFormatter) -> FormatResult<()> {
        let TsModuleBlockFields {
            l_curly_token,
            items,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [format_delimited(&l_curly_token?, &items.format(), &r_curly_token?,).block_indent()]
        )
    }
}
