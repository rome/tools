use crate::prelude::*;

use rome_formatter::{format_args, write};
use rome_js_syntax::{
    JsAnyExpression, JsxAnyTag, JsxExpressionAttributeValue, JsxExpressionAttributeValueFields,
};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxExpressionAttributeValue;

impl FormatNodeRule<JsxExpressionAttributeValue> for FormatJsxExpressionAttributeValue {
    fn fmt_fields(
        &self,
        node: &JsxExpressionAttributeValue,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsxExpressionAttributeValueFields {
            l_curly_token,
            expression,
            r_curly_token,
        } = node.as_fields();

        let expression = expression?;

        let should_inline = should_inline_jsx_expression(&expression);

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

/// Tests if an expression inside of a [JsxExpressionChild] or [JsxExpressionAttributeValue] should be inlined.
/// Good:
/// ```jsx
///  <ColorPickerPage
///     colors={[
///        "blue",
///        "brown",
///        "green",
///        "orange",
///        "purple",
///     ]} />
/// ```
///
/// Bad:
/// ```jsx
///  <ColorPickerPage
///     colors={
///       [
///         "blue",
///          "brown",
///         "green",
///         "orange",
///         "purple",
///       ]
///     } />
/// ```
pub(crate) fn should_inline_jsx_expression(expression: &JsAnyExpression) -> bool {
    use JsAnyExpression::*;

    if expression.syntax().has_comments_direct() {
        return false;
    }

    match expression {
        JsArrayExpression(_)
        | JsObjectExpression(_)
        | JsArrowFunctionExpression(_)
        | JsCallExpression(_)
        | JsNewExpression(_)
        | JsImportCallExpression(_)
        | ImportMeta(_)
        | JsFunctionExpression(_)
        | JsTemplate(_) => true,
        JsAwaitExpression(await_expression) => match await_expression.argument() {
            Ok(JsxTagExpression(argument)) => {
                matches!(argument.tag(), Ok(JsxAnyTag::JsxElement(_)))
                    && should_inline_jsx_expression(&argument.into())
            }
            _ => false,
        },
        _ => false,
    }
}
