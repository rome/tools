use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_syntax::JsThisExpression;
use rslint_syntax::JsThisExpressionFields;

impl ToFormatElement for JsThisExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsThisExpressionFields { this_token } = self.as_fields();

        this_token.format(formatter)
    }
}
