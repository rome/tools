use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::ConcatBuilder;
use rome_js_syntax::{
    JsStaticMemberExpression, JsStaticMemberExpressionFields, JsSyntaxKind, JsSyntaxNode,
    JsSyntaxToken,
};
use rome_rowan::AstNode;

impl FormatNodeFields<JsStaticMemberExpression> for FormatNodeRule<JsStaticMemberExpression> {
    fn format_fields(
        node: &JsStaticMemberExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let mut current = node.clone();

        while let Some(static_member_expression) =
            JsStaticMemberExpression::cast(current.object()?.syntax().clone())
        {
            current = static_member_expression;
        }

        let JsStaticMemberExpressionFields {
            object,
            operator_token,
            member,
        } = current.as_fields();

        let mut formatted = ConcatBuilder::default();

        let operator_token = operator_token?;
        let formatted_object = format_member(object?.syntax().clone(), &operator_token, formatter);

        formatted.entry(formatted![
            formatter,
            [formatted_object, operator_token.format()]
        ]?);

        let mut previous_member = member;

        while let Some(parent_static_member_expression) = current
            .syntax()
            .parent()
            .and_then(JsStaticMemberExpression::cast)
        {
            let JsStaticMemberExpressionFields {
                object: _,
                operator_token,
                member,
            } = parent_static_member_expression.as_fields();

            let operator_token = operator_token?;

            let formatted_member = format_member(
                previous_member?.syntax().clone(),
                &operator_token,
                formatter,
            )?;

            formatted.entry(group_elements(formatted![
                formatter,
                [formatted_member, operator_token.format(),]
            ]?));

            previous_member = member;
            current = parent_static_member_expression;
        }

        formatted.entry(formatted![formatter, [previous_member.format()?,]]?);
        Ok(group_elements(formatted.finish()))
    }
}

fn format_member(
    member: JsSyntaxNode,
    operator: &JsSyntaxToken,
    formatter: &JsFormatter,
) -> FormatResult<FormatElement> {
    let formatted_member = formatted![formatter, [member.format()]]?;

    let is_object_number_literal = member.kind() == JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION;

    let object_has_trailing_trivia = member
        .last_trailing_trivia()
        .map_or(false, |trivia| trivia.pieces().len() > 0);

    let operator_has_leading_trivia = operator.leading_trivia().pieces().len() > 0;

    if is_object_number_literal && (object_has_trailing_trivia || operator_has_leading_trivia) {
        let (object_leading, object_content, object_trailing) = formatted_member.split_trivia();

        Ok(formatted![
            formatter,
            [
                object_leading,
                token("("),
                object_content,
                token(")"),
                object_trailing,
            ]
        ]?)
    } else {
        Ok(formatted_member)
    }
}
