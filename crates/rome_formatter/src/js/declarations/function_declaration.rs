use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use crate::formatter_traits::FormatTokenAndNode;
use rome_js_syntax::{JsAnyFunction, JsFunctionDeclaration};

impl ToFormatElement for JsFunctionDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        JsAnyFunction::from(self.clone()).format(formatter)
    }
}
