use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_syntax::JsAnyClass;
use rslint_syntax::JsClassExportDefaultDeclaration;

impl ToFormatElement for JsClassExportDefaultDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        JsAnyClass::from(self.clone()).format(formatter)
    }
}
