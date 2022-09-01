use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{
    JsAnyExpression, JsxExpressionAttributeValue, JsxExpressionAttributeValueFields,
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
        write!(
            f,
            [group(&format_with(|f| {
                write!(f, [l_curly_token.format()])?;

                let expression = expression.as_ref()?;

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
                if matches!(
                    expression,
                    JsAnyExpression::JsObjectExpression(_)
                        | JsAnyExpression::JsArrayExpression(_)
                        | JsAnyExpression::JsCallExpression(_)
                        | JsAnyExpression::JsArrowFunctionExpression(_)
                ) {
                    write!(f, [expression.format()])?;
                } else {
                    write!(f, [soft_block_indent(&expression.format())])?;
                };

                write!(f, [line_suffix_boundary(), r_curly_token.format()])
            }))]
        )
    }
}
