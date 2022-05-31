use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::ConcatBuilder;
use rome_js_syntax::{JsSequenceExpression, JsSequenceExpressionFields, JsSyntaxKind};
use rome_rowan::AstNode;

impl FormatNodeFields<JsSequenceExpression> for FormatNodeRule<JsSequenceExpression> {
    fn format_fields(
        node: &JsSequenceExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let mut current = node.clone();
        let parent = current.syntax().parent();

        let has_already_indentation = parent.map_or(false, |parent| {
            // Return statement already does the indentation for us
            // Arrow function body can't have a sequence expression unless it's parenthesized, otherwise
            // would be a syntax error
            if matches!(parent.kind(), JsSyntaxKind::JS_RETURN_STATEMENT) {
                true
            } else if matches!(parent.kind(), JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION) {
                // In case we are inside a sequence expression, we have to go up a level and see the great parent.
                // Arrow function body and return statements applying indentation for us, so we signal the
                // sequence expression to not add other indentation levels
                let great_parent = parent.parent().map(|gp| gp.kind());

                matches!(
                    great_parent,
                    Some(
                        JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
                            | JsSyntaxKind::JS_RETURN_STATEMENT
                            | JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER
                    )
                )
            } else {
                false
            }
        });

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

        let mut formatted = ConcatBuilder::default();

        formatted.entry(formatted![
            formatter,
            [left.format()?, comma_token.format()?]
        ]?);

        let mut previous_right = right;

        // Traverse upwards again and concatenate the sequence expression until we find the first non-sequence expression
        while let Some(parent_sequence) = current
            .syntax()
            .parent()
            .and_then(JsSequenceExpression::cast)
        {
            let JsSequenceExpressionFields {
                left: _left,
                comma_token,
                right,
            } = parent_sequence.as_fields();

            if has_already_indentation {
                formatted.entry(formatted![
                    formatter,
                    [
                        soft_line_break_or_space(),
                        previous_right.format()?,
                        comma_token.format()?,
                    ]
                ]?);
            } else {
                formatted.entry(formatted![
                    formatter,
                    [indent(formatted![
                        formatter,
                        [
                            soft_line_break_or_space(),
                            previous_right.format()?,
                            comma_token.format()?,
                        ]
                    ]?),]
                ]?)
            }
            previous_right = right;
            current = parent_sequence;
        }

        if has_already_indentation {
            formatted.entry(formatted![
                formatter,
                [soft_line_break_or_space(), previous_right.format()?,]
            ]?);
        } else {
            formatted.entry(formatted![
                formatter,
                [indent(formatted![
                    formatter,
                    [soft_line_break_or_space(), previous_right.format()?,]
                ]?),]
            ]?)
        }

        Ok(group_elements(formatted.finish()))
    }
}
