use crate::prelude::*;

use rome_js_syntax::JsShorthandPropertyObjectMember;
use rome_js_syntax::JsShorthandPropertyObjectMemberFields;

impl FormatNode for JsShorthandPropertyObjectMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsShorthandPropertyObjectMemberFields { name } = self.as_fields();

        name.format(formatter)
    }
}
