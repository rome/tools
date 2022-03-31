use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsSuperExpression;
use rome_js_syntax::JsSuperExpressionFields;

impl ToFormatElement for JsSuperExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsSuperExpressionFields { super_token } = self.as_fields();

        super_token.format(formatter)
    }
}
