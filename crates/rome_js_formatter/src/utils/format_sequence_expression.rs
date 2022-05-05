use crate::{
    concat_elements, format_elements, group_elements, Format, FormatElement, FormatResult,
    Formatter,
};
use rome_formatter::{soft_block_indent, soft_line_break_or_space};
use rome_js_syntax::{JsSequenceExpression, JsSequenceExpressionFields};
use rome_rowan::AstNode;

#[derive(Eq, PartialEq)]
pub(crate) enum SequenceContext {
    InFunction,
    None,
}

pub(crate) fn format_sequence_expression(
    node: &JsSequenceExpression,
    formatter: &Formatter,
    sequence_context: SequenceContext,
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

    let mut formatted = vec![format_elements![
        left.format(formatter)?,
        comma_token.format(formatter)?
    ]];

    let mut previous_right = right;

    // Traverse upwards again and concatenate the sequence expression until we find the first non-sequence expression
    while let Some(parent) = current.syntax().parent() {
        if let Some(parent_sequence) = JsSequenceExpression::cast(parent) {
            let JsSequenceExpressionFields {
                left: _left,
                comma_token,
                right,
            } = parent_sequence.as_fields();

            if sequence_context == SequenceContext::InFunction {
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
        } else {
            break;
        }
    }

    if sequence_context == SequenceContext::InFunction {
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
