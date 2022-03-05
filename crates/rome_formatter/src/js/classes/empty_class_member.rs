use crate::{empty_element, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_syntax::JsEmptyClassMember;
use rslint_syntax::JsEmptyClassMemberFields;

impl ToFormatElement for JsEmptyClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsEmptyClassMemberFields { semicolon_token } = self.as_fields();

        formatter.format_replaced(&semicolon_token?, empty_element())
    }
}
