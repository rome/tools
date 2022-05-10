use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::{JsSequenceExpression, JsSequenceExpressionFields};
use rome_rowan::AstNode;

impl FormatNodeFields<JsSequenceExpression> for FormatNodeRule<JsSequenceExpression> {
    fn format_fields(
        node: &JsSequenceExpression,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        let mut current = node.clone();

        // Find the left most sequence expression
        while let Some(sequence_expression) =
            JsSequenceExpression::cast(current.left()?.syntax().clone())
        {
            current = sequence_expression;
        }

        // Format the left most sequence expression
        let JsSequenceExpressionFields {
            left,
            comma_token,
            right,
        } = current.as_fields();

        let mut formatted = vec![formatted![
            formatter,
            left.format(),
            comma_token.format(),
            space_token(),
            right.format(),
        ]?];

        // Traverse upwards again and concatenate the sequence expression until we find the first non-sequence expression
        while let Some(parent) = current.syntax().parent() {
            if let Some(parent_sequence) = JsSequenceExpression::cast(parent) {
                let JsSequenceExpressionFields {
                    left: _left,
                    comma_token,
                    right,
                } = parent_sequence.as_fields();

                formatted.push(formatted![
                    formatter,
                    comma_token.format(),
                    space_token(),
                    right.format()
                ]?);

                current = parent_sequence;
            } else {
                break;
            }
        }

        Ok(concat_elements(formatted))
    }
}
