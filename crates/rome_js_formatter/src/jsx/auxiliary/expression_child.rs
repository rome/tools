use crate::jsx::attribute::expression_attribute_value::should_inline_jsx_expression;
use crate::prelude::*;
use crate::prelude::{format_args, write};

use crate::utils::JsAnyBinaryLikeExpression;
use rome_formatter::{group, FormatResult};
use rome_js_syntax::{JsAnyExpression, JsxExpressionChild, JsxExpressionChildFields};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxExpressionChild;

impl FormatNodeRule<JsxExpressionChild> for FormatJsxExpressionChild {
    fn fmt_fields(&self, node: &JsxExpressionChild, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxExpressionChildFields {
            l_curly_token,
            expression,
            r_curly_token,
        } = node.as_fields();

        let should_inline = expression.as_ref().map_or(true, |expression| {
            if matches!(expression, JsAnyExpression::JsConditionalExpression(_))
                || JsAnyBinaryLikeExpression::can_cast(expression.syntax().kind())
            {
                true
            } else {
                should_inline_jsx_expression(expression)
            }
        });

        if should_inline {
            write!(
                f,
                [
                    l_curly_token.format(),
                    expression.format(),
                    line_suffix_boundary(),
                    r_curly_token.format()
                ]
            )
        } else {
            write!(
                f,
                [group(&format_args![
                    l_curly_token.format(),
                    soft_block_indent(&expression.format()),
                    line_suffix_boundary(),
                    r_curly_token.format()
                ])]
            )
        }
    }
}
