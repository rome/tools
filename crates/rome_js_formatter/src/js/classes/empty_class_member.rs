use crate::prelude::*;

use rome_js_syntax::JsEmptyClassMember;
use rome_js_syntax::JsEmptyClassMemberFields;

impl FormatNode for JsEmptyClassMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsEmptyClassMemberFields { semicolon_token } = self.as_fields();

        Ok(formatter.format_replaced(&semicolon_token?, empty_element()))
    }
}
