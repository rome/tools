use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_syntax::JsAnyFunction;
use rslint_syntax::JsFunctionExportDefaultDeclaration;

impl ToFormatElement for JsFunctionExportDefaultDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        JsAnyFunction::from(self.clone()).format(formatter)
    }
}
