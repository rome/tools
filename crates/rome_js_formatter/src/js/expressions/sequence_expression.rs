use crate::utils::{format_sequence_expression, SequenceContext};
use crate::{FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::JsSequenceExpression;

impl FormatNodeFields<JsSequenceExpression> for FormatNodeRule<JsSequenceExpression> {
    fn format_fields(
        node: &JsSequenceExpression,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        format_sequence_expression(self, formatter, SequenceContext::None)
    }
}

pub(crate) fn format_sequence_expression(
    node: &JsSequenceExpression,
    formatter: &Formatter,
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
                    JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION | JsSyntaxKind::JS_RETURN_STATEMENT
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

    let mut formatted = vec![format_elements![
        left.format(formatter)?,
        comma_token.format(formatter)?
    ]];

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
            formatted.push(format_elements![
                soft_line_break_or_space(),
                previous_right.format(formatter)?,
                comma_token.format(formatter)?,
            ]);
        } else {
            formatted.push(soft_block_indent(format_elements![
                soft_line_break_or_space(),
                previous_right.format(formatter)?,
                comma_token.format(formatter)?,
            ]))
        }
        previous_right = right;
        current = parent_sequence;
    }

    if has_already_indentation {
        formatted.push(format_elements![
            soft_line_break_or_space(),
            previous_right.format(formatter)?,
        ]);
    } else {
        formatted.push(soft_block_indent(format_elements![
            soft_line_break_or_space(),
            previous_right.format(formatter)?,
        ]))
    }

    Ok(group_elements(concat_elements(formatted)))
}
