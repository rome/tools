use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use crate::formatter_traits::FormatTokenAndNode;
use rome_js_syntax::{JsAnyClass, JsClassExpression};

impl ToFormatElement for JsClassExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        JsAnyClass::from(self.clone()).format(formatter)
    }
}
