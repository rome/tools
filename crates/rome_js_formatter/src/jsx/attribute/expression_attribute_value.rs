use crate::{
    formatted, soft_block_indent, Format, FormatElement, FormatNode, Formatter,
};
use rome_formatter::{group_elements, FormatResult};
use rome_js_syntax::{
    JsAnyExpression, JsxExpressionAttributeValue, JsxExpressionAttributeValueFields,
};

impl FormatNode for JsxExpressionAttributeValue {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxExpressionAttributeValueFields {
            l_curly_token,
            expression,
            r_curly_token,
        } = self.as_fields();

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
            expression.format(formatter)?
        } else {
            soft_block_indent(expression.format(formatter)?)
        };

        Ok(group_elements(formatted![
            formatter,
            l_curly_token.format(formatter)?,
            formatted_expression,
            r_curly_token.format(formatter)?,
        ]?))
    }
}
