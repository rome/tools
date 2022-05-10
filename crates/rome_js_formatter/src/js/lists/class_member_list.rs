use crate::generated::FormatJsClassMemberList;
use crate::prelude::*;
use rome_js_syntax::JsClassMemberList;

impl FormatRule<JsClassMemberList> for FormatJsClassMemberList {
    type Options = JsFormatOptions;

    fn format(
        node: &JsClassMemberList,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        Ok(formatter.format_list_with_hard_line(node))
    }
}
