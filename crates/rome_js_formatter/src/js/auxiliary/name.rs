use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsName;
use rome_js_syntax::JsNameFields;

impl ToFormatElement for JsName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsNameFields { value_token } = self.as_fields();

        value_token.format(formatter)
    }
}
