use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_syntax::JsShorthandPropertyObjectMember;
use rslint_syntax::JsShorthandPropertyObjectMemberFields;

impl ToFormatElement for JsShorthandPropertyObjectMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsShorthandPropertyObjectMemberFields { name } = self.as_fields();

        name.format(formatter)
    }
}
