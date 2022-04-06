use crate::{FormatElement, FormatResult, Formatter, ToFormatElement, formatter_traits::FormatTokenAndNode};
use rome_js_syntax::{AstNode, JsxTagExpression};
impl ToFormatElement for JsxTagExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.tag().format(formatter)
    }
}
