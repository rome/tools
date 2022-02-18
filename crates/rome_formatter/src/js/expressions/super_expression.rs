use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsSuperExpression;
use rslint_parser::ast::JsSuperExpressionFields;

impl ToFormatElement for JsSuperExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsSuperExpressionFields { super_token } = self.as_fields();

        super_token.format(formatter)
    }
}
