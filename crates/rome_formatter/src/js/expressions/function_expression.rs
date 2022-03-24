use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use crate::formatter_traits::FormatTokenAndNode;
use rome_js_syntax::{JsAnyFunction, JsFunctionExpression};

impl ToFormatElement for JsFunctionExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        JsAnyFunction::from(self.clone()).format(formatter)
    }
}
