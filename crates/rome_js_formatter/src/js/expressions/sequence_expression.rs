use rome_formatter::concat_elements;

use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};

use rome_js_syntax::{JsSequenceExpression, JsSequenceExpressionFields};
use rome_rowan::AstNode;

impl FormatNode for JsSequenceExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let mut current = self.clone();

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

        let mut formatted = vec![
            left.format(formatter)?,
            comma_token.format(formatter)?,
            space_token(),
            right.format(formatter)?,
        ];

        // Traverse upwards again and concatenate the sequence expression until we find the first non-sequence expression
        while let Some(parent) = current.syntax().parent() {
            if let Some(parent_sequence) = JsSequenceExpression::cast(parent) {
                let JsSequenceExpressionFields {
                    left: _left,
                    comma_token,
                    right,
                } = parent_sequence.as_fields();

                formatted.push(format_elements![
                    comma_token.format(formatter)?,
                    space_token(),
                    right.format(formatter)?
                ]);

                current = parent_sequence;
            } else {
                break;
            }
        }

        Ok(concat_elements(formatted))
    }
}
