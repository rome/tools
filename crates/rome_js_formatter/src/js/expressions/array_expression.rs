use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsArrayExpression;
use rome_js_syntax::JsArrayExpressionFields;

impl FormatNode for JsArrayExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
