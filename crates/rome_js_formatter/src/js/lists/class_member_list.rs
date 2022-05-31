use crate::formatter::TryFormatNodeListExtension;
use crate::generated::FormatJsClassMemberList;
use crate::prelude::*;
use rome_js_syntax::JsClassMemberList;

impl FormatRule<JsClassMemberList> for FormatJsClassMemberList {
    type Context = JsFormatContext;

    fn format(node: &JsClassMemberList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&hard_line_break())
            .entries(node.try_format_nodes())
            .finish()
    }
}
