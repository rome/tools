use crate::generated::FormatJsClassMemberList;
use crate::prelude::*;
use rome_js_syntax::JsClassMemberList;

impl FormatRule<JsClassMemberList> for FormatJsClassMemberList {
    type Context = JsFormatContext;

    fn format(node: &JsClassMemberList, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(node))
    }
}
