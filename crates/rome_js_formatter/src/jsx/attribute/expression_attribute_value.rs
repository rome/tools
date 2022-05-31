use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{
    JsAnyExpression, JsxExpressionAttributeValue, JsxExpressionAttributeValueFields,
};

impl FormatNodeFields<JsxExpressionAttributeValue> for FormatNodeRule<JsxExpressionAttributeValue> {
    fn format_fields(
        node: &JsxExpressionAttributeValue,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsxExpressionAttributeValueFields {
            l_curly_token,
            expression,
            r_curly_token,
        } = node.as_fields();

        let expression = expression?;

        // When the inner expression for a prop is an object, array, or call expression, we want to combine the
        // delimiters of the expression (`{`, `}`, `[`, `]`, or `(`, `)`) with the delimiters of the JSX
        // attribute (`{`, `}`), so that we don't end up with redundant indents. Therefore we do not
        // soft indent the expression
        //
        // Good:
        // ```jsx
        //  <ColorPickerPage
        //     colors={[
        //        "blue",
        //        "brown",
        //        "green",
        //        "orange",
        //        "purple",
        //     ]} />
        // ```
        //
        // Bad:
        // ```jsx
        //  <ColorPickerPage
        //     colors={
        //       [
        //         "blue",
        //          "brown",
        //         "green",
        //         "orange",
        //         "purple",
        //       ]
        //     } />
        // ```
        //
        let formatted_expression = if matches!(
            expression,
            JsAnyExpression::JsObjectExpression(_)
                | JsAnyExpression::JsArrayExpression(_)
                | JsAnyExpression::JsCallExpression(_)
        ) {
            formatted![formatter, [expression.format()]]?
        } else {
            soft_block_indent(formatted![formatter, [expression.format()]]?)
        };

        Ok(group_elements(formatted![
            formatter,
            [
                l_curly_token.format(),
                formatted_expression,
                line_suffix_boundary(),
                r_curly_token.format(),
            ]
        ]?))
    }
}
