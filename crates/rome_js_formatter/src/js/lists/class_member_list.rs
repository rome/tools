use crate::prelude::*;
use rome_js_syntax::JsClassMemberList;
impl Format for JsClassMemberList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(self.clone()))
    }
}
