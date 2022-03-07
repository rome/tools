use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsIdentifierExpression;
use rome_js_syntax::JsIdentifierExpressionFields;

impl ToFormatElement for JsIdentifierExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsIdentifierExpressionFields { name } = self.as_fields();

        name.format(formatter)
    }
}
