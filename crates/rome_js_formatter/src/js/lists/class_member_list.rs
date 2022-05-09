use crate::generated::FormatJsClassMemberList;
use crate::prelude::*;
use rome_js_syntax::JsClassMemberList;

impl FormatRule<JsClassMemberList> for FormatJsClassMemberList {
    fn format(node: &JsClassMemberList, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(node))
    }
}
