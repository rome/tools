use crate::prelude::*;
use rome_js_syntax::TsEnumMemberList;

#[derive(Debug, Clone, Default)]
pub struct FormatTsEnumMemberList;

impl FormatRule<TsEnumMemberList> for FormatTsEnumMemberList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &TsEnumMemberList, f: &mut JsFormatter) -> FormatResult<()> {
        let mut joiner = f.join_nodes_with_soft_line();

        for variant in node.format_separated(",").nodes_grouped() {
            joiner.entry(variant.node()?.syntax(), &variant)
        }

        joiner.finish()
    }
}
