use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsArrayExpression;
use rslint_parser::ast::JsArrayExpressionFields;

impl ToFormatElement for JsArrayExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsArrayExpressionFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = self.as_fields();

        formatter.format_delimited_soft_block_indent(
            &l_brack_token?,
            elements.format(formatter)?,
            &r_brack_token?,
        )
    }
}
